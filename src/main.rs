use std::io::Result;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
// use listenfd::ListenFd;

mod controllers;

use controllers::{time, weather};

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let host = std::env::var("HOST").expect("$HOST is not set");
    let port: u16 = std::env::var("PORT").expect("$PORT is not set").parse().unwrap();
    HttpServer::new(|| {
        App::new()
            .configure(weather::init_routes)
            .configure(time::init_routes)
    })
        .bind((host, port)).expect("Unable to bind to address.")
        .run()
        .await
}
