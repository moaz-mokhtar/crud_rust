# crud_rust

A demo project which shows sample CRUD operations with Rust.
Model is Driver for sample drivers.

## Technologies

Technologies are as below:

- Rust programming language #rust_lang
- Actix web framework
- Postgres for database
- Diesel ORM
- REST API

## Development Guidance

To run the project you need to have Docker, then run below:

```bash
docker compose up 
```

To run tests, preferred to run them consecutively

```bash
cargo test -- --test-threads=1
```

### prerequisites

- Install Rust follow link: <https://www.rust-lang.org/tools/install>
- Install Docker follow link: <https://docs.docker.com/get-docker/>
- install Diesel CLI, link: <https://crates.io/crates/diesel_cli>

### Database

- Run database in docker

```bash
docker compose up db
```

- install `diesel` and database 

```bash
# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres
# Create an empty migrations directory that we can use to manage our schema
diesel setup
# Generate migration script with name
diesel migration generate <MIGRATION_NAME>
# Run migration scripts
diesel migration run
```

- Run migration to the db

```bash
# Make sure that Postgres db running simply by command
docker compose up db test_db

# Run migration for normal db
diesel migration run

# Run migration for test db
diesel migration run --database-url postgres://username:password@localhost:5433/test_db

# ======
# In case you wan to reset db and test db do below:
# Database reset for normal db
diesel database reset

# Database reset for test db
diesel database reset  --database-url postgres://username:password@localhost:5433/test_db
```

## REST APIs

There are 5 endpoints as below:
- 