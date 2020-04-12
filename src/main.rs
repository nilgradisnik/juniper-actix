#[macro_use]
extern crate log;

use std::io;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};

mod graphql;
mod schema;
mod sqlite;

use crate::graphql::{graphql, playground};
use crate::schema::create_schema;

const SERVER: &'static str = "127.0.0.1:8080";

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(Cors::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::post().to(graphql)))
            .service(web::resource("/playground").route(web::get().to(playground)))
    })
    .bind(SERVER)?
    .run()
    .await
}
