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
# Run in from docker
docker compose up server

# Run curl commands to test apis
sh curl_apis.sh
```

To run tests, preferred to run them consecutively

```bash
# Run tests from docker
docker compose up test_server

```

### prerequisites

- Install Rust follow link: <https://www.rust-lang.org/tools/install>
- Install Docker follow link: <https://docs.docker.com/get-docker/>
- install Diesel CLI, link: <https://crates.io/crates/diesel_cli>

### Database

- Run database in docker

```bash
docker compose up db test_db
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

# Note: You can run dababases with migrations from docker as below
docker compose up db test_db db-migrations

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

```bash
# Health check
curl -X GET http://localhost:$PORT/api/

# Create new driver
curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"first_name": "Moaz","last_name": "Mokhtar", "email": "moaz.mokhtar@gmail.com", "phone": "0154864354"}' \
    http://localhost:$PORT/api/drivers

# Get all drivers
curl -X GET http://localhost:$PORT/api/drivers

# Get a driver
curl -X GET http://localhost:$PORT/api/drivers/<DRIVER_UUID>

# Delete a driver
curl -X DELETE http://localhost:$PORT/api/drivers/<DRIVER_UUID>


```