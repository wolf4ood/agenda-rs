use std::str::FromStr;

use agenda_domain::{
    errors::DomainError,
    todos::{NewTodo, Todo, TodoId, TodoRepo, TodoStatus},
};

use structsy::Structsy;
use structsy_derive::{queries, Persistent, PersistentEmbedded};
pub struct TodoRepoStrucsty {
    db: Structsy,
}
use structsy::StructsyTx;
use uuid::Uuid;

impl TodoRepoStrucsty {
    pub fn open(url: &str) -> Self {
        let db = Structsy::open(url).unwrap();
        db.define::<PersistentTodo>().unwrap();
        TodoRepoStrucsty { db }
    }
}

#[async_trait::async_trait]
impl TodoRepo for TodoRepoStrucsty {
    async fn create(&self, todo: NewTodo) -> Result<Todo, DomainError> {
        let id = TodoId::generate();

        let mut tx = self.db.begin().unwrap();

        let persistent_todo = PersistentTodo {
            id: id.take().to_string(),
            title: todo.title().clone(),
            description: todo.description().clone(),
            status: todo.status().clone().into(),
        };
        tx.insert(&persistent_todo).unwrap();

        tx.commit().unwrap();
        Ok(persistent_todo.into())
    }

    async fn get(&self, id: &TodoId) -> Result<Option<Todo>, DomainError> {
        Ok(self
            .db
            .query::<PersistentTodo>()
            .by_id(id.to_string())
            .into_iter()
            .next()
            .map(|(_, data)| data.into()))
    }

    async fn delete(&self, _id: &TodoId) -> Result<Option<Todo>, DomainError> {
        todo!()
    }
    async fn all(&self) -> Result<Vec<Todo>, DomainError> {
        self.db
            .scan::<PersistentTodo>()
            .unwrap()
            .into_iter()
            .map(|(_, data)| Ok(data.into()))
            .collect()
    }

    async fn update(&self, _todo: &Todo) -> Result<Todo, DomainError> {
        todo!()
    }
}

#[derive(Persistent)]
struct PersistentTodo {
    #[index(mode = "exclusive")]
    id: String,
    pub title: String,
    pub description: String,
    pub status: PersistentTodoStatus,
}
#[derive(PersistentEmbedded)]
enum PersistentTodoStatus {
    Active,
    Completed,
}

impl From<TodoStatus> for PersistentTodoStatus {
    fn from(status: TodoStatus) -> Self {
        match status {
            TodoStatus::Active => PersistentTodoStatus::Active,
            TodoStatus::Completed => PersistentTodoStatus::Completed,
        }
    }
}

#[queries(PersistentTodo)]
trait TodoQuery {
    fn by_id(self, id: String) -> Self;
}

impl From<PersistentTodo> for Todo {
    fn from(todo: PersistentTodo) -> Self {
        Self {
            id: Uuid::from_str(todo.id.as_str()).unwrap().into(),
            title: todo.title,
            description: todo.description,
            status: todo.status.into(),
        }
    }
}

impl From<PersistentTodoStatus> for TodoStatus {
    fn from(status: PersistentTodoStatus) -> Self {
        match status {
            PersistentTodoStatus::Active => TodoStatus::Active,
            PersistentTodoStatus::Completed => TodoStatus::Completed,
        }
    }
}
