use crate::schema;
use crate::schema::AgendaSchema;
use actix_web::{guard, middleware::Logger, web, App, HttpResponse, HttpServer};
use async_graphql::{Request, Variables};
use serde::Deserialize;

// GRAPHQL
pub async fn start_graphql_app() -> std::io::Result<()> {
    env_logger::init();
    let client = reqwest::Client::new();
    let schema = schema(client);

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(Logger::default())
            .service(web::resource("/graphql").guard(guard::Post()).to(graphql))
    })
    .bind("127.0.0.1:8082")?
    .workers(20)
    .run()
    .await
}
#[derive(Deserialize)]
pub struct GraphQLQuery {
    query: String,
    variables: Variables,
}
pub async fn graphql(
    schema: web::Data<AgendaSchema>,
    query: web::Json<GraphQLQuery>,
) -> HttpResponse {
    let req = Request::new(query.query.clone()).variables(query.variables.clone());

    let res = schema.execute(req).await;

    HttpResponse::Ok()
        .content_type("application/json")
        .json(res)
}
