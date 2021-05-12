use crate::types::{Todo, TodoStatus};
use async_graphql::*;
use serde::Serialize;
use uuid::Uuid;

use reqwest::Client;

pub struct Query;

#[Object]
impl Query {
    async fn todo(&self, ctx: &Context<'_>, todo_id: Uuid) -> Result<Todo> {
        let client = ctx.data_unchecked::<Client>();
        let response: Todo = client
            .get(format!("http://localhost:8081/todos/{}", todo_id))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        Ok(response)
    }

    async fn todos(&self, ctx: &Context<'_>) -> Result<Vec<Todo>> {
        let client = ctx.data_unchecked::<Client>();
        let response: Vec<Todo> = client
            .get("http://localhost:8081/todos")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        Ok(response)
    }
}

pub struct Mutation;

#[derive(Serialize)]
pub struct NewTodoData {
    title: String,
    description: String,
}
#[Object]
impl Mutation {
    async fn create_todo(
        &self,
        ctx: &Context<'_>,
        title: String,
        description: String,
    ) -> Result<Todo> {
        let client = ctx.data_unchecked::<Client>();
        let response: Todo = client
            .post("http://localhost:8081/todos")
            .json(&NewTodoData { title, description })
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        Ok(response)
    }

    async fn delete_todo(&self, _todo_id: Uuid) -> Result<Todo> {
        todo!()
    }

    async fn change_status(&self, _todo_id: Uuid, _status: TodoStatus) -> Result<Todo> {
        todo!()
    }
}

pub type AgendaSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn schema(client: reqwest::Client) -> AgendaSchema {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(client)
        .finish()
}

impl From<agenda_domain::todos::Todo> for Todo {
    fn from(todo: agenda_domain::todos::Todo) -> Self {
        Todo {
            id: todo.id.take(),
            title: todo.title,
            description: todo.description,
            status: TodoStatus::Active,
        }
    }
}
