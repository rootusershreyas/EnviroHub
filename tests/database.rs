use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use envirohub::{app, db, state::AppState};
use tower::ServiceExt;

#[tokio::test]
async fn postgres_connection_and_readiness_follow_pool_availability() {
    // Export DATABASE_URL explicitly for tests; the binary's dotenv loading does
    // not run when Cargo starts integration-test executables.
    let database_url = std::env::var("DATABASE_URL")
        .expect("Export DATABASE_URL and start PostgreSQL before running database tests");
    let pool = db::connect(&database_url, 2).await.unwrap();
    let value: i32 = sqlx::query_scalar("SELECT 1")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(value, 1);

    let router = app::router(AppState { pool: pool.clone() });
    let response = router.clone().oneshot(get("/ready")).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), 1024).await.unwrap();
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&body).unwrap(),
        serde_json::json!({"status": "ready"})
    );

    // Closing the pool simulates loss of database access without stopping the
    // shared development container. The process itself must remain healthy.
    pool.close().await;
    let response = router.clone().oneshot(get("/ready")).await.unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = to_bytes(response.into_body(), 1024).await.unwrap();
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&body).unwrap(),
        serde_json::json!({
            "code": "service_unavailable",
            "message": "Database is temporarily unavailable"
        })
    );
    assert_eq!(
        router.oneshot(get("/health")).await.unwrap().status(),
        StatusCode::OK
    );
}

fn get(path: &str) -> Request<Body> {
    Request::builder().uri(path).body(Body::empty()).unwrap()
}
