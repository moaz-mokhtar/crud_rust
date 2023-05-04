use actix_web::{
    web::{self, ServiceConfig},
    Error, HttpResponse, Responder,
};
use log::info;
use uuid::Uuid;

use crate::{
    db::DbPool,
    models::{DefaultResponse, Driver, NewDriverRequest},
};

pub fn routes_config(config: &mut ServiceConfig) {
    config.service(
        web::scope("/api")
            .service(web::resource("/").route(web::get().to(health)))
            .service(
                web::scope("/drivers")
                    .route("", web::post().to(new_driver))
                    .route("", web::get().to(get_all))
                    .route("/{driverid}", web::get().to(get_driver))
                    .route("/{driverid}", web::delete().to(delete_driver))
                    .route("/rand100", web::get().to(new_100_driver))
                    .route("/all_by_name/", web::get().to(get_all_by_name))
                    .route("/all_by_char/", web::get().to(get_all_by_char)),
            ),
    );
}

pub async fn health() -> impl Responder {
    info!("Health check");

    let response = DefaultResponse {
        description: "Ok".to_string(),
    };
    HttpResponse::Ok().json(response)
}

pub async fn get_driver(
    path: web::Path<i32>,
    _pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let driver_id: i32 = path.into_inner();
    info!("Get driver: {}", driver_id);

    todo!("get_driver")
}

pub async fn get_all(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    info!("Get all drivers");

    match Driver::get_all(&pool).await {
        Ok(drivers) => Ok(HttpResponse::Ok().json(drivers)),
        Err(e) => Ok(HttpResponse::NotFound().json(e.to_string())),
    }
}

pub async fn get_all_by_name(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    info!("Get all drivers sorted by name");

    match Driver::get_all_by_name(&pool).await {
        Ok(drivers) => Ok(HttpResponse::Ok().json(drivers)),
        Err(e) => Ok(HttpResponse::NotFound().json(e.to_string())),
    }
}

pub async fn get_all_by_char(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    info!("Get all drivers but names sorted based on characters");

    match Driver::get_all_by_char(&pool).await {
        Ok(drivers) => Ok(HttpResponse::Ok().json(drivers)),
        Err(e) => Ok(HttpResponse::NotFound().json(e.to_string())),
    }
}

pub async fn new_driver(
    data: web::Json<NewDriverRequest>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    info!("New driver: {:?}", data.0.clone());

    match Driver::insert(data.0, &pool).await {
        Ok(new_driver) => Ok(HttpResponse::Ok().json(new_driver)),
        Err(e) => Ok(HttpResponse::BadRequest().json(DefaultResponse {
            description: e.to_string(),
        })),
    }
}

pub async fn new_100_driver(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    info!("Insert random 100 drivers");

    let drivers_list = Driver::mocks(100);

    match Driver::insert_bulk(drivers_list, &pool).await {
        Ok(affected_rows) => Ok(HttpResponse::Ok().json(affected_rows)),
        Err(e) => Ok(HttpResponse::BadRequest().json(DefaultResponse {
            description: e.to_string(),
        })),
    }
}

pub async fn delete_driver(
    path: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let driver_id: Uuid = path.into_inner();
    info!("Delete driver: {}", driver_id);

    match Driver::delete(driver_id, &pool).await {
        Ok(_) => Ok(HttpResponse::Ok().json(DefaultResponse {
            description: "Driver deleted".to_string(),
        })),
        Err(e) => Ok(HttpResponse::NotFound().json(e.to_string())),
    }
}
