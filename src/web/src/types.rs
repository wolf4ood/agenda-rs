use async_graphql::*;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use uuid::Uuid;

#[derive(SimpleObject, Builder, Debug)]
pub struct Todo {
    id: Uuid,
    title: String,
    description: String,
    status: TodoStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum TodoStatus {
    Active,
    Completed,
}
