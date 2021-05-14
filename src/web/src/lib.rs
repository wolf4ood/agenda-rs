use actix_web::{middleware::Logger, web, App, HttpServer};
use agenda_strucsty::TodoRepoStrucsty;
mod context;
mod error;
mod proxy;
mod rest;
mod schema;
mod types;
pub mod utils;
pub use agenda_db::{PgPool, TodoRepoDiesel};
pub use context::ApplicationContext;
pub use proxy::start_graphql_app;
pub use schema::schema;

pub async fn start_rest_app(db_url: &str) -> std::io::Result<()> {
    env_logger::init();
    let todos = TodoRepoStrucsty::open(db_url);
    let ctx = ApplicationContext::new(todos);

    HttpServer::new(move || {
        App::new()
            .data(ctx.clone())
            .wrap(Logger::default())
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
