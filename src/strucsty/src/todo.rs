use agenda_domain::{
    errors::DomainError,
    todos::{NewTodo, Todo, TodoId, TodoRepo},
};

use structsy::Structsy;
pub struct TodoRepoStrucsty {
    db: Structsy,
}

impl TodoRepoStrucsty {
    pub fn open(url: &str) -> Self {
        TodoRepoStrucsty {
            db: Structsy::open(url).unwrap(),
        }
    }
}

#[async_trait::async_trait]
impl TodoRepo for TodoRepoStrucsty {
    async fn create(&self, todo: NewTodo) -> Result<Todo, DomainError> {
        let id = TodoId::generate();

        todo!()
    }

    async fn get(&self, id: &TodoId) -> Result<Option<Todo>, DomainError> {
        todo!()
    }

    async fn delete(&self, id: &TodoId) -> Result<Option<Todo>, DomainError> {
        todo!()
    }
    async fn all(&self) -> Result<Vec<Todo>, DomainError> {
        todo!()
    }

    async fn update(&self, todo: &Todo) -> Result<Todo, DomainError> {
        todo!()
    }
}
