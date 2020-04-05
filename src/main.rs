use actix_web::{web, middleware, App, HttpServer};

#[macro_use] extern crate log;

mod graphql;
mod schema;
mod sqlite;

const SERVER: &'static str = "127.0.0.1:8080";

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // HttpServer::new(move || {
    //     App::new()
    //         .data(db_pool.clone())
    //         .wrap(Cors::new())
    //         .configure(graphql::register)
    //         .default_service(web::to(|| "404"))
    // })
    // .bind(addr)
    // .unwrap()
    // .run()
    // .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(graphql::register)
            .default_service(web::to(|| "404"))
    })
    .bind(SERVER)
    .unwrap()
    .run()
    .unwrap();
}
