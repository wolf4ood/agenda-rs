use crate::{types::Todo, ApplicationContext};
use actix_web::{get, post, web, HttpResponse};
use agenda_domain::todos::NewTodo;
use serde::Deserialize;
use uuid::Uuid;

#[get("")]
pub async fn show_todos(ctx: web::Data<ApplicationContext>) -> HttpResponse {
    match ctx.todos().all().await {
        Ok(todos) => HttpResponse::Ok().json(todos.into_iter().map(Todo::from).collect::<Vec<_>>()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
#[post("")]
pub async fn create_todo(
    data: web::Json<NewTodoWeb>,
    ctx: web::Data<ApplicationContext>,
) -> HttpResponse {
    match NewTodo::create(data.title.clone(), data.description.clone()) {
        Ok(todo) => match ctx.todos().create(todo).await {
            Ok(created) => HttpResponse::Created().json(Todo::from(created)),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
pub async fn todo_details(id: web::Path<Uuid>, ctx: web::Data<ApplicationContext>) -> HttpResponse {
    match ctx.todos().get(&id.into_inner().into()).await {
        Ok(Some(res)) => HttpResponse::Ok().json(Todo::from(res)),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_err) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize)]
pub struct NewTodoWeb {
    title: String,
    description: String,
}
