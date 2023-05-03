use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use drivers::{db::DbClient, handler};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Welcome to CRUD demo with Rust");
    initiate_logging();

    let pool = DbClient::get_pool_connection();
    let pool_test = DbClient::get_test_pool();

    let data = Data::new(pool);
    let data_test = Data::new(pool_test);

    let server_address =
        std::env::var("SERVER_HOST").expect("Missed 'SERVER_HOST' environment variable");
    info!("Starting server at {}", server_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .app_data(data.clone())
            .app_data(data_test.clone())
            .service(web::scope("").configure(handler::routes_config))
    })
    .bind(server_address)?
    .run()
    .await
}

/// Initialize logging and confirm .env file is present
pub fn initiate_logging() {
    std::env::set_var("RUST_LOG", "debug, actix_web=debug");

    let env = dotenv::from_filename(".env").expect("'.env' not found.");
    dbg!(env);

    if std::env::var("PWD").is_err() {
        std::env::set_var("PWD", env!("CARGO_MANIFEST_DIR"));
        let pwd = std::env::var("PWD").unwrap();
        dbg!(pwd);
    }

    env_logger::init();
}
