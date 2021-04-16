use agenda_domain::{
    errors::DomainError,
    todos::{NewTodo, Todo, TodoId, TodoRepo},
};

pub struct TodoRepoInMemory {}

impl TodoRepoInMemory {
    pub fn new() -> Self {
        TodoRepoInMemory {}
    }
}

#[async_trait::async_trait]
impl TodoRepo for TodoRepoInMemory {
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

    async fn update(&self, _todo: &Todo) -> Result<(), DomainError> {
        todo!()
    }
}
