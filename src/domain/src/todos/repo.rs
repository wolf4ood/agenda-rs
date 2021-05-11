use crate::{
    errors::DomainError,
    todos::{Todo, TodoId},
};

use super::models::NewTodo;

#[async_trait::async_trait]
pub trait TodoRepo: Send + Sync {
    async fn create(&self, todo: NewTodo) -> Result<Todo, DomainError>;
    async fn get(&self, id: &TodoId) -> Result<Option<Todo>, DomainError>;

    async fn all(&self) -> Result<Vec<Todo>, DomainError>;

    async fn delete(&self, id: &TodoId) -> Result<Option<Todo>, DomainError>;

    async fn update(&self, todo: &Todo) -> Result<Todo, DomainError>;
}
