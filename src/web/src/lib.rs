use actix_web::{guard, middleware::Logger, web, App, HttpResponse, HttpServer};
use agenda_strucsty::TodoRepoStrucsty;
use async_graphql::{Request, Variables};
use schema::AgendaSchema;
use tracing_actix_web::TracingLogger;
mod context;
mod error;
mod rest;
mod schema;
mod types;
pub mod utils;
pub use agenda_db::{PgPool, TodoRepoDiesel};
pub use context::ApplicationContext;
pub use schema::schema;
use serde::Deserialize;
pub async fn start_graphql_app() -> std::io::Result<()> {
    utils::install_telemetry("graphql_service").unwrap();

    let client = reqwest::Client::new();
    let schema = schema(client);

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(Logger::default())
            .wrap(TracingLogger::default())
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

pub async fn start_rest_app(db_url: &str) -> std::io::Result<()> {
    utils::install_telemetry("rest_service").unwrap();
    let todos = TodoRepoStrucsty::open(db_url);
    let ctx = ApplicationContext::new(todos);

    HttpServer::new(move || {
        App::new()
            .data(ctx.clone())
            .wrap(Logger::default())
            .wrap(TracingLogger::default())
            .service(
                web::scope("todos")
                    .service(rest::show_todos)
                    .service(rest::todo_details)
                    .service(rest::create_todo),
            )
    })
    .bind("127.0.0.1:8081")?
    .workers(20)
    .run()
    .await
}
