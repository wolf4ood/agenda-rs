use agenda_web::start_rest_app;
use dotenv::dotenv;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("env DATABASE_URL not found");

    tracing_subscriber::fmt::init();

    start_rest_app(db_url.as_str()).await
}
