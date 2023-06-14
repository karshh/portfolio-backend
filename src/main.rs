use std::io::Result;
use actix_web::{App, HttpServer};
// use listenfd::ListenFd;

mod controllers;

use controllers::{time, weather};

#[actix_rt::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(weather::init_routes)
            .configure(time::init_routes)
    })
        .bind(("127.0.0.1", 8090)).expect("Unable to bind to address.")
        .run()
        .await
}
