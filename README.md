# crud_rust
A demo project which shows sample CRUD operations with Rust

## Technologies
Technologies are as below:
- Rust programming language #rust_lang
- Actix web framework
- Postgres for database
- Diesel ORM
- REST API

## Development Guidance:

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
