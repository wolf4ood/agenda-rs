use actix_web::{get, post, web, HttpResponse};
use uuid::Uuid;

#[get("")]
pub async fn show_todos() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}
#[post("")]
pub async fn create_todo() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}

#[get("/id}")]
pub async fn todo_details(path: web::Path<(Uuid,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
}
