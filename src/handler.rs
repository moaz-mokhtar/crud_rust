use actix_web::{ web::{ ServiceConfig, self }, Responder, HttpResponse, Error };
use log::info;

use crate::{ models::{ DefaultResponse, NewDriverRequest }, db::DbPool };

pub fn routes_config(config: &mut ServiceConfig) {
    config.service(
        web
            ::scope("/v1")
            .service(web::resource("/").route(web::get().to(health)))
            .service(
                web
                    ::scope("/drivers")
                    .route("/{driverid}", web::get().to(get_driver))
                    .route("", web::get().to(get_all))
                    .route("", web::post().to(new_driver))
                    .route("/{driverid}", web::delete().to(delete_driver))
            )
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
    _pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let driver_id: i32 = path.into_inner();
    info!("Get driver: {}", driver_id);

    todo!("get_driver")
}

pub async fn get_all(_pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    info!("Get all drivers");

    todo!("get_all")

    // match Uploadeddriver::get_all(&pool).await {
    //     Ok(drivers) => Ok(HttpResponse::Ok().json(drivers)),
    //     Err(e) => Ok(HttpResponse::NotFound().json(e.to_string())),
    // }
}

pub async fn new_driver(
    _data: web::Json<NewDriverRequest>,
    _pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    info!("New driver");

    todo!("new_driver")
}

pub async fn delete_driver(
    path: web::Path<i32>,
    _pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let driver_id: i32 = path.into_inner();
    info!("Delete driver: {}", driver_id);

    todo!("delete_driver")
}