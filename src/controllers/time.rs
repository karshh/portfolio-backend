use actix_web::{HttpResponse, web, get, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Test {
    name: String,
}

#[get("/time")]
async fn get_time() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(
        Test {
            name: String::from("The time is now!!")
        }
    ))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_time);
}