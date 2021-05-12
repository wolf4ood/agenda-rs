use agenda_web::start_rest_app;
use dotenv::dotenv;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("STRUCTSY_URL").expect("env STRUCTSY_URL not found");

    start_rest_app(db_url.as_str()).await
}
