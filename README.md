# EnviroHub

A shared environment booking API built as one Rust application. This first
milestone provides Member A's application foundation.

## Run

Install stable Rust, then run:

```sh
cargo run
```

The server defaults to `127.0.0.1:3000`. Configure it with environment variables:

```sh
BIND_ADDR=0.0.0.0:3000 RUST_LOG=envirohub=info,tower_http=debug cargo run
```

`.env.example` documents the settings; the application reads process environment
variables and does not automatically load a `.env` file.

In another terminal:

```sh
curl http://127.0.0.1:3000/health
# {"status":"ok"}
```

`GET /health` is a process health check and requires no database. Logs are JSON.
Ctrl+C or SIGTERM initiates graceful shutdown.

## Verify

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Integration boundaries

- `src/app.rs` exposes `router()` for application startup and HTTP tests. Add
  feature routers under `/api/v1` here.
- `src/error.rs` maps shared application errors to JSON containing `code` and
  `message`, with HTTP 400, 404, 409, and 500 responses.
- Member B adds SQLx, PostgreSQL configuration, migrations, and database readiness.
  Coordinate adding a shared `AppState` containing the pool at that point.
- Member A adds programmers, teams, memberships, and lead rules next.
- Member C adds environments and the booking lifecycle.

Authentication, database access, and feature endpoints are not part of this
foundation milestone.

## Project context

This is a three-person, two-week learning project based on P6. The application foundation above is implemented; feature work below remains planned unless checked off.

## Project scope

Implement the P6 programmer, team, project, and task APIs, then connect them to a shared-environment booking workflow.

1. Register programmers and organize them into teams.
2. Assign a team lead and create projects owned by teams.
3. Create tasks and assign them to members of the project's team.
4. Register shared environments and view their availability.
5. Reserve an environment for a project and a time window.
6. Reject overlapping reservations; support cancellation and rescheduling.

The first version records reservations. Automatic provisioning, deployment access enforcement, notifications, recurring bookings, and a dashboard are outside this iteration. Authentication/authorization is deferred as instructed in P6.

## Technology stack and planned integrations

| Technology | Purpose |
| --- | --- |
| Rust and Tokio | Domain types, ownership, async runtime |
| Axum | RESTful HTTP API |
| SQLx and PostgreSQL | Async database access, migrations, transactions and constraints |
| Serde and JSON | Request and response serialization |
| tracing and log | Structured logging and request context |
| Utoipa / OpenAPI | API documentation |
| Rust integration tests | Test HTTP behavior with real PostgreSQL |
| Docker / Compose | Reproducible local application and database setup |
| GitHub Actions | Automated formatting, linting and tests |
| xh and jq | Manual API testing and demonstration |

Async will let the server handle requests while waiting for database I/O. PostgreSQL constraints and transactions will enforce cross-request rules such as preventing double booking; Rust ownership does not replace database concurrency control.

## Three-person task division

Assign names to A, B and C before implementation. The repository owner also takes one workstream. Each contributor implements their feature's code, validation, error handling, tests and OpenAPI documentation.

### Contributor A — Programmers and teams

- [x] Initialize the Rust/Axum application, configuration, routing and shared error format in an early setup PR.
- [ ] Register, retrieve, list and update programmers; support deactivation.
- [ ] Create, retrieve, list and update teams.
- [ ] Add, list and remove team members; allow membership in multiple teams.
- [ ] Assign/change team leads. Every team must have exactly one lead who is a member.
- [ ] Reject removal of a current lead unless a replacement is assigned atomically.
- [ ] Agree and test deactivation/member-removal behavior with B and C so assignments and bookings remain consistent.

Suggested branches: `feat/api-setup`, `feat/programmers`, `feat/teams`.

### Contributor B — Projects and tasks

- [ ] Set up SQLx/PostgreSQL and coordinate the initial schema/migrations with A and C in an early PR.
- [ ] Create, retrieve, list and update projects, including owning team and status.
- [ ] List projects belonging to a team.
- [ ] Implement task creation, retrieval, updates and deletion.
- [ ] Include every P6 task field: project, title, description, status, priority, optional assignee/due date, timestamps and version.
- [ ] Assign/reassign tasks only to members of the project's owning team; allow unassigned tasks.
- [ ] List tasks by project and programmer; support filtering, sorting and pagination.
- [ ] Implement task version checks so competing updates cannot silently overwrite each other.
- [ ] Define and test project team changes when assigned tasks or bookings already exist.

Suggested branches: `feat/database-setup`, `feat/projects`, `feat/tasks`.

### Contributor C — Environments and bookings

- [ ] Set up Docker/Compose and CI; coordinate structured request logging with A in an early PR.
- [ ] Register, retrieve, list and update environments, including availability/maintenance status.
- [ ] Create bookings linked to an environment, project and requesting programmer.
- [ ] Validate time ranges, requester membership and environment eligibility.
- [ ] Prevent overlapping reservations using a PostgreSQL exclusion constraint; return `409 Conflict` for overlap.
- [ ] Implement availability lookup, booking retrieval/listing, cancellation and rescheduling.
- [ ] Preserve cancelled booking history and use version checks for competing booking updates.
- [ ] Test simultaneous conflicting requests, adjacent slots, cancellation/rebooking and failed rescheduling.

Suggested branches: `chore/docker-ci`, `feat/environments`, `feat/bookings`.

## Shared agreements before coding

- Agree the schema, endpoint paths, JSON format, error codes and pagination convention.
- Retain P6's programmer, team, project and task fields and status values.
- Use timestamps with an explicit offset and store booking times as PostgreSQL `timestamptz`.
- Use half-open booking windows: 10:00–11:00 and 11:00–12:00 may coexist.
- Agree transaction/locking rules for operations that affect membership, assignment and booking eligibility.
- Add tests and API documentation in the same PR as each feature.
- Coordinate edits to shared routing, dependencies and migrations; avoid duplicate setup work.

## Two-week implementation plan

| Time | Deliverable |
| --- | --- |
| Day 1 | Assign workstreams, agree contracts and merge small setup PRs |
| Days 2–3 | Programmer/team, project and environment basics |
| Days 4–5 | Task operations and initial bookings; integrate an end-to-end flow |
| Days 6–7 | Cancellation/rescheduling, filtering/pagination, version conflicts and business-rule checks |
| Day 8 | Real-PostgreSQL integration tests, concurrency tests and defect fixes |
| Day 9 | Complete OpenAPI, setup instructions and xh/jq demonstration |
| Day 10 | Final verification, review and evaluation rehearsal |

## Branch and PR workflow

Use `develop` as the integration branch. All three members create their feature branches from `develop` and target `develop` with their PRs. After combined testing and team review, merge `develop` into `main` for a release. Do not push feature work directly to `main`.

```bash
git clone https://github.com/rootusershreyas/EnviroHub.git
cd EnviroHub
git switch develop
git pull --ff-only origin develop
git switch -c feat/programmers
# Implement your task, tests and documentation.
git add <changed-files>
git commit -m "feat(programmers): add registration and retrieval"
git push -u origin feat/programmers
gh pr create --base develop --fill
```

Another teammate reviews each PR before merge. Suggested review rotation: A reviews B, B reviews C, C reviews A. Keep `develop` working, integrate regularly, and pull its latest changes before starting the next task.

## Completion checklist

- [ ] All original P6 programmer, team, project and task requirements are implemented.
- [ ] Invalid lead removal and out-of-team task assignment are rejected.
- [ ] Task filtering, sorting, pagination and version conflicts work.
- [ ] Overlapping bookings are rejected even under simultaneous HTTP requests.
- [ ] Availability, cancellation and rescheduling work without corrupting existing reservations.
- [ ] Structured logs and OpenAPI cover the implemented API.
- [ ] Integration tests use real PostgreSQL and pass in CI.
- [ ] Docker setup and README run instructions are verified from a clean checkout.
- [ ] The team can demonstrate the complete workflow using xh and jq.

## Evaluation demonstration

Create programmers and a team → designate a lead → create a project and task → assign a team member → register staging → reserve it → reject a competing reservation → cancel and reserve the released slot.

Also demonstrate a stale task-version conflict, invalid assignment/lead removal, structured request logs and passing database integration tests.
