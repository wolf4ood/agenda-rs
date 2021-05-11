use actix_web::{guard, middleware::Logger, web, App, HttpServer};
use agenda_memory::TodoRepoInMemory;
use async_graphql_actix_web::{Request, Response};
use schema::AgendaSchema;
mod context;
mod schema;
mod types;
pub use agenda_db::{PgPool, TodoRepoDiesel};
pub use context::ApplicationContext;
pub use schema::schema;

pub async fn start_app(db_url: &str) -> std::io::Result<()> {
    // let _pool = PgPool::new(db_url);
    // let todos = TodoRepoInMemory::new();
    // let ctx = ApplicationContext::new(todos);
    // let schema = schema(ctx);

    HttpServer::new(move || {
        App::new()
            // .data(schema.clone())
            .wrap(Logger::default())
            .service(web::resource("/graphql").guard(guard::Post()).to(graphql))
    })
    .bind("127.0.0.1:8080")?
    .workers(20)
    .run()
    .await
}

pub async fn graphql(schema: web::Data<AgendaSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}
