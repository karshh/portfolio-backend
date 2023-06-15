use actix_web::{HttpResponse, web, get};
use serde::Serialize;

use crate::services::temperature;

#[derive(Debug, Serialize)]
struct WeatherResponse {
    temperature: f32
}

#[get("/weather")]
async fn get_weather() -> HttpResponse {
    match temperature::get_current_temperature().await {
    Err(_) => HttpResponse::InternalServerError().finish(),
    Ok(temperature) => HttpResponse::Ok().json(
            WeatherResponse {
                temperature
            }
        )
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_weather);
}