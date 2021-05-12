use async_graphql::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(SimpleObject, Builder, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: TodoStatus,
}

#[derive(Enum, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
pub enum TodoStatus {
    Active,
    Completed,
}
