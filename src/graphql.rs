use std::sync::Arc;

use actix_web::{web, Error, HttpResponse};

use juniper::http::GraphQLRequest;
use juniper::http::playground::playground_source;

use crate::sqlite::Sqlite;
use crate::schema::Schema;

pub async fn playground() -> HttpResponse {
    let html = playground_source("");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let body = web::block(move || {
        let context = Sqlite{ };
        let res = data.execute(&st, &context);

        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}
