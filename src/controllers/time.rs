use actix_web::{HttpResponse, web, get};
use actix_web::cookie::time::OffsetDateTime;
use serde::Serialize;

use crate::services::time::get_current_time;

#[derive(Debug, Serialize)]
struct TimeResponse {
    #[serde(with = "time::serde::rfc3339")]
    datetime: OffsetDateTime,
}

#[get("/time")]
async fn get_time() -> HttpResponse {
    match get_current_time().await {
        Ok(datetime) => HttpResponse::Ok().json( TimeResponse { datetime } ),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_time);
}