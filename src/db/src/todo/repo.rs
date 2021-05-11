use agenda_domain::{
    errors::DomainError,
    todos::{NewTodo, Todo, TodoId, TodoRepo},
};

use crate::PgPool;
use anyhow::Result;
pub struct TodoRepoDiesel(PgPool);

impl TodoRepoDiesel {
    pub fn new(pool: PgPool) -> Self {
        TodoRepoDiesel(pool)
    }
}

#[async_trait::async_trait]
impl TodoRepo for TodoRepoDiesel {
    async fn create(&self, _todo: NewTodo) -> Result<Todo, DomainError> {
        todo!()
    }

    async fn get(&self, _id: &TodoId) -> Result<Option<Todo>, DomainError> {
        todo!()
    }

    async fn delete(&self, _id: &TodoId) -> Result<Option<Todo>, DomainError> {
        todo!()
    }
    async fn all(&self) -> Result<Vec<Todo>, DomainError> {
        todo!()
    }

    async fn update(&self, _todo: &Todo) -> Result<Todo, DomainError> {
        todo!()
    }
}
