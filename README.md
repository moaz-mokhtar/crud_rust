# crud_rust

A demo project which shows sample CRUD operations with Rust.

## Features
- CRUD operations for model `Driver`
- Engine functions (dealing with db transactions) covered with unit tests
- System seperated into modules
- Created simple mock generator functions, check: https://github.com/moaz-mokhtar/mock-generator
- Can run the system by docker or locally

## Technologies

Technologies are as below:

- Server by Rust programming language #rust_lang
- Actix web framework
- Postgres for database
- Diesel ORM for database transactions
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

```bash
# Health check
curl -X GET http://localhost:8080/api/

# Create new driver
curl -X POST \
    -H "Content-Type: application/json" \
    -d '{"first_name": "Moaz","last_name": "Mokhtar", "email": "moaz.mokhtar@gmail.com", "phone": "0154864354"}' \
    http://localhost:8080/api/drivers

# Get all drivers
curl -X GET http://localhost:8080/api/drivers

# Get a driver
curl -X GET http://localhost:8080/api/drivers/<DRIVER_UUID>

# Delete a driver
curl -X DELETE http://localhost:8080/api/drivers/<DRIVER_UUID>

# Get list of randomly generated 100 dirivers
curl -X GET http://localhost:8080/api/drivers/rand100

# Get list of drivers sorted by name
curl -X GET http://localhost:8080/api/drivers/all_by_name

# Get list of drivers sorted by name's charaters
curl -X GET http://localhost:8080/api/drivers/all_by_char

```


--- 
Developed by `Moaz in Mokhtar`
