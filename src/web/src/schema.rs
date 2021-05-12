use crate::types::{Todo, TodoStatus};
use async_graphql::*;
use serde::Serialize;
use uuid::Uuid;

pub struct Query;

#[Object]
impl Query {
    async fn todo(&self, _todo_id: Uuid) -> Result<Todo> {
        todo!()
    }

    async fn todos(&self) -> Result<Vec<Todo>> {
        todo!()
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
    #[tracing::instrument(skip(self))]
    async fn create_todo(&self, title: String, description: String) -> Result<Todo> {
        let client = reqwest::Client::new();
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

pub fn schema() -> AgendaSchema {
    Schema::build(Query, Mutation, EmptySubscription).finish()
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
