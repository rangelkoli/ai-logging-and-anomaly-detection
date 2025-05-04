## Phase 0: Setup & Basics (Estimated Time: 1-2 Weeks)

- [x] Set up Rust development environment (compiler, Cargo).
- [x] Set up your preferred code editor with rust-analyzer.
- [x] Create a new Rust project using `cargo new log_analyzer_saas`.
- [x] Practice Rust basics (variables, types, functions, control flow) within the `main.rs` file.
- [x] Write a function that reads lines from a local sample log file (e.g., nginx access log, simple JSON logs).

## Phase 1: Core Data Structures & Log Parsing (Estimated Time: 2-3 Weeks)

- [x] Define Rust structs to represent parsed log entries (e.g., `ParsedLogEntry`).
- [x] Define Rust enums where appropriate (e.g., `LogLevel`).
- [x] Implement parser function(s) `fn parse_line(line: &str) -> Result<ParsedLogEntry, ParseError>`.
- [x] Ensure the parser handles at least one common log format (e.g., Apache/Nginx common, structured JSON).
- [x] Implement logic to read a sample log file line by line.
- [x] Parse each line using your parser function.
- [x] Store the successfully parsed entries in a `Vec<ParsedLogEntry>`.
- [ ] Implement basic unit tests for the parser function(s).
- [ ] Ensure unit tests cover handling of malformed or invalid log lines.

## Phase 2: Error Handling, Modularity & Basic Storage (Estimated Time: 2-3 Weeks)

- [ ] Refactor the log parsing logic into its own separate Rust module.
- [ ] Define custom error types (e.g., `enum AppError`, `enum ParseError`, `enum StorageError`).
- [ ] Improve error handling throughout the parsing and file reading logic using `Result` and the `?` operator.
- [ ] Define a `LogStorage` trait with methods like `store(log: ParsedLogEntry)` and `query(filter: QueryFilter) -> Result<Vec<ParsedLogEntry>, StorageError>`. (Define a basic `QueryFilter` struct/enum too).
- [ ] Implement a simple `FileLogStorage` struct that implements the `LogStorage` trait.
- [ ] Use `serde` (with `serde_json` or `bincode`) to serialize `ParsedLogEntry` data for storage in the `FileLogStorage`.
- [ ] Implement the `store` method for `FileLogStorage` (e.g., append serialized data to a file).
- [ ] Implement a basic `query` method for `FileLogStorage` (e.g., reads the whole file, deserializes, and filters in memory).

## Phase 3: Web API & Concurrent Ingestion (Estimated Time: 3-4 Weeks)

- [ ] Choose a web framework (e.g., `axum` or `actix-web`) and add it as a dependency.
- [ ] Choose an async runtime (likely `tokio`) and set up the basic `main` function for an async server.
- [ ] Wrap your `LogStorage` implementation (e.g., `FileLogStorage`) in an `Arc<Mutex<...>>` or `Arc<RwLock<...>>` for thread-safe shared access.
- [ ] Create an HTTP endpoint (e.g., `POST /api/v1/logs`).
- [ ] Implement the handler for the ingestion endpoint to:
  - [ ] Accept log lines in the request body (e.g., as JSON).
  - [ ] Deserialize the incoming request body.
  - [ ] Parse the log line(s) using your existing parser module.
  - [ ] Store the parsed log(s) using the shared, thread-safe `LogStorage` instance.
- [ ] Ensure the server can handle concurrent requests safely to the ingestion endpoint.
- [ ] Add basic logging to the web server application using the `log` or `tracing` crate.

## Phase 4: Scalable Storage & Querying (Estimated Time: 3-5 Weeks)

- [ ] Choose a database strategy (e.g., PostgreSQL with `sqlx`, MongoDB with `mongodb`, SQLite with `sqlx`, RocksDB).
- [ ] Define a database schema (SQL) or document structure (NoSQL) for storing `ParsedLogEntry` data.
- [ ] Ensure appropriate database indexes are defined (e.g., on timestamp, important fields).
- [ ] Add the chosen database driver crate (e.g., `sqlx`, `mongodb`) as a dependency.
- [ ] Implement a new `DbLogStorage` struct that implements the `LogStorage` trait.
- [ ] Implement the `store` method for `DbLogStorage` to insert log data into the database. Use connection pooling if applicable (e.g., `sqlx` provides it).
- [ ] Implement the `query` method for `DbLogStorage` efficiently using database features (e.g., `WHERE` clauses, index utilization).
- [ ] Create an API endpoint (e.g., `GET /api/v1/logs`).
- [ ] Implement the handler for the query endpoint to:
  - [ ] Accept filter parameters (e.g., time range, keywords, field values) from the query string or request body.
  - [ ] Use the `query` method of `DbLogStorage` to retrieve matching logs.
  - [ ] Implement pagination for the query results.
  - [ ] Return the queried logs (and pagination info) as a JSON response.
- [ ] Replace the `Arc<Mutex<FileLogStorage>>` with `Arc<DbLogStorage>` (or similar) in your web server setup.

## Phase 5: Anomaly Detection & Background Tasks (Estimated Time: 4-6 Weeks)

- [ ] Design basic statistical anomaly detection logic (e.g., threshold checks, standard deviation checks on log counts/frequencies).
- [ ] Implement functions that query the database (via `DbLogStorage`) to get the necessary data for anomaly detection.
- [ ] Implement the core anomaly detection algorithm(s).
- [ ] Create a background task runner using `tokio::spawn`.
- [ ] Implement logic within the background task to periodically (e.g., every minute using `tokio::time::interval`) trigger the anomaly detection process on recent logs.
- [ ] Define a way to store detected anomalies (e.g., a new database table or collection).
- [ ] Implement logic to store detected anomalies.
- [ ] Create an API endpoint (e.g., `GET /api/v1/alerts`) to retrieve stored anomalies.
- [ ] Make anomaly detection rules configurable (e.g., load thresholds/parameters from a config file or database table).
- [ ] (Optional) Explore using `pyo3` to integrate a Python ML model:
  - [ ] Set up `pyo3` build environment.
  - [ ] Write Rust code to call a Python function/model.
  - [ ] Handle data marshalling between Rust and Python.

## Phase 6: SaaS Features, Polish & Deployment Prep (Estimated Time: 4-6 Weeks+)

- [ ] Design database schema modifications for multi-tenancy (e.g., add `user_id` or `org_id` to logs, rules, alerts tables/collections).
- [ ] Update all database storage and query logic (`DbLogStorage`) to enforce data isolation based on the tenant/user.
- [ ] Choose and add dependencies for authentication (e.g., `jsonwebtoken`, `argon2`/`bcrypt`).
- [ ] Implement user signup endpoint (e.g., `POST /api/v1/users`) including password hashing.
- [ ] Implement user login endpoint (e.g., `POST /api/v1/auth/login`) that returns a JWT upon success.
- [ ] Implement web framework middleware to verify JWTs and protect relevant API endpoints.
- [ ] Extract user/tenant information from the validated token in protected endpoints.
- [ ] Choose and add a rate limiting crate (e.g., `governor`).
- [ ] Apply rate limits to appropriate API endpoints (e.g., login, ingestion).
- [ ] Choose and add a configuration management crate (e.g., `config`).
- [ ] Externalize all configuration (database URLs, secrets, ports, JWT secret, etc.) using the chosen crate.
- [ ] Choose and add a metrics crate (e.g., `prometheus`).
- [ ] Instrument the application to expose metrics (e.g., request counts, durations, errors, logs processed).
- [ ] Create a `Dockerfile` to build a container image for the application.
- [ ] Write API documentation (e.g., using OpenAPI specifications, maybe generated from code).
- [ ] Refine error handling across the entire application for clarity and user-friendliness.
- [ ] Add more comprehensive tests (integration tests, potentially end-to-end tests).
- [ ] Use `rustdoc` to generate documentation for your crates/modules.
