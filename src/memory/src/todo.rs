use agenda_domain::{
    errors::DomainError,
    todos::{NewTodo, Todo, TodoId, TodoRepo},
};
use std::collections::HashMap;
use std::sync::RwLock;
pub struct TodoRepoInMemory {
    todos: RwLock<HashMap<TodoId, Todo>>,
}

impl TodoRepoInMemory {
    pub fn new() -> Self {
        TodoRepoInMemory {
            todos: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl TodoRepo for TodoRepoInMemory {
    async fn create(&self, todo: NewTodo) -> Result<Todo, DomainError> {
        let id = TodoId::generate();
        let stored = Todo {
            id: id.clone(),
            title: todo.title().clone(),
            description: todo.description().clone(),
            status: todo.status().clone(),
        };

        let mut todos = self.todos.write().unwrap();

        todos.insert(id, stored.clone());

        Ok(stored)
    }

    async fn get(&self, id: &TodoId) -> Result<Option<Todo>, DomainError> {
        let todos = self.todos.read().unwrap();

        Ok(todos.get(id).cloned())
    }

    async fn delete(&self, id: &TodoId) -> Result<Option<Todo>, DomainError> {
        let mut todos = self.todos.write().unwrap();

        Ok(todos.remove(id))
    }
    async fn all(&self) -> Result<Vec<Todo>, DomainError> {
        let todos = self.todos.read().unwrap();

        Ok(todos.values().cloned().collect())
    }

    async fn update(&self, todo: &Todo) -> Result<Todo, DomainError> {
        let mut todos = self.todos.write().unwrap();

        todos
            .entry(todo.id.clone())
            .and_modify(|p_todo| *p_todo = todo.clone());

        Ok(todo.clone())
    }
}
