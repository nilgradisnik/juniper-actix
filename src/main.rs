use std::io;
use std::sync::Arc;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};

use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;

#[macro_use] extern crate log;

mod schema;
mod sqlite;

use crate::schema::{create_schema, Schema};
use crate::sqlite::Sqlite;

const SQLITE_DB: &'static str = "./sqlite.db";
const SERVER: &'static str = "127.0.0.1:8080";

async fn playground() -> HttpResponse {
    let endpoint = format!("http://{server}/graphql", server = SERVER);
    let html = playground_source(&endpoint);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let context = Sqlite{ db_path: SQLITE_DB };
        let res = data.execute(&st, &context);

        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/playground").route(web::get().to(playground)))
    })
    .bind(SERVER)?
    .run()
    .await
}
