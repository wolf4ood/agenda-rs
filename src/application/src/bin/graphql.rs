use agenda_web::start_graphql_app;
use dotenv::dotenv;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    start_graphql_app().await
}
