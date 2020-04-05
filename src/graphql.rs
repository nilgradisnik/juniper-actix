use std::sync::Arc;

use actix_web::{web, Error, HttpResponse};
use futures::future::Future;

use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;

use crate::schema::Schema;
use crate::sqlite::Sqlite;

fn playground() -> HttpResponse {
    let html = playground_source("");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphql(
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // let user = web::block(move || {
    //     let context = Sqlite{ };
    //     let res = data.execute(&schema, &context);

    //     Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    // })
    // .await?;

    // Ok(HttpResponse::Ok()
    //     .content_type("application/json")
    //     .body(user))

    let context = Sqlite{ };

    web::block(move || {
        let res = data.execute(&schema, &context);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

pub fn register(config: &mut web::ServiceConfig) {
    let schema = std::sync::Arc::new(Schema::create_schema);

    config
        .data(schema)
        .route("/", web::post().to_async(graphql))
        .route("/", web::get().to(playground));
}
