use async_graphql::*;
use derive_builder::Builder;
use uuid::Uuid;

#[derive(SimpleObject, Builder, Debug)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: TodoStatus,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum TodoStatus {
    Active,
    Completed,
}
