use actix_web::{HttpResponse, web, get, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Test {
    name: String,
}

#[get("/weather")]
async fn get_weather() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(
        Test {
            name: String::from("Hello world!")
        }
    ))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_weather);
}