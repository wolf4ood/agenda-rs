use crate::{error::MyError, types::Todo, ApplicationContext};
use actix_web::{get, post, web, HttpResponse};
use agenda_domain::todos::NewTodo;
use anyhow::Context;
use serde::Deserialize;
use uuid::Uuid;

#[get("")]
pub async fn show_todos(ctx: web::Data<ApplicationContext>) -> Result<HttpResponse, MyError> {
    let todos = ctx.todos().all().await.context("Failed to get all todos")?;
    Ok(HttpResponse::Ok().json(todos.into_iter().map(Todo::from).collect::<Vec<_>>()))
}
#[post("")]
pub async fn create_todo(
    data: web::Json<NewTodoWeb>,
    ctx: web::Data<ApplicationContext>,
) -> Result<HttpResponse, MyError> {
    let todo = NewTodo::create(data.title.clone(), data.description.clone())
        .context("Failed to create todo")?;
    let created = ctx
        .todos()
        .create(todo)
        .await
        .context("Failed to store todo")?;

    Ok(HttpResponse::Created().json(Todo::from(created)))
}

#[get("/{id}")]
pub async fn todo_details(
    id: web::Path<Uuid>,
    ctx: web::Data<ApplicationContext>,
) -> Result<HttpResponse, MyError> {
    match ctx
        .todos()
        .get(&id.into_inner().into())
        .await
        .context("Failed to get todo")?
    {
        Some(res) => Ok(HttpResponse::Ok().json(Todo::from(res))),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[derive(Deserialize)]
pub struct NewTodoWeb {
    title: String,
    description: String,
}
