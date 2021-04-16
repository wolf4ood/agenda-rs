use uuid::Uuid;

use crate::errors::DomainError;

use super::TodoRepo;

#[derive(Debug, PartialEq)]
pub struct TodoId(Uuid);

impl TodoId {
    pub fn generate() -> TodoId {
        TodoId(Uuid::new_v4())
    }
}

impl From<Uuid> for TodoId {
    fn from(id: Uuid) -> Self {
        TodoId(id)
    }
}

#[derive(Debug)]
pub struct Todo {
    id: TodoId,
    title: String,
    description: String,
    status: TodoStatus,
}

impl Todo {
    /// Get a reference to the todo's id.
    pub fn id(&self) -> &TodoId {
        &self.id
    }

    /// Get a reference to the todo's text.
    pub fn title(&self) -> &String {
        &self.title
    }

    /// Get a reference to the todo's status.
    pub fn status(&self) -> &TodoStatus {
        &self.status
    }
    /// Get a reference to the new todo's description.
    pub fn description(&self) -> &String {
        &self.description
    }

    pub async fn change_status(
        self,
        _status: TodoStatus,
        _repo: &impl TodoRepo,
    ) -> Result<Self, DomainError> {
        todo!()
    }

    pub async fn mark_completed(self, repo: &impl TodoRepo) -> Result<Self, DomainError> {
        self.change_status(TodoStatus::Completed, repo).await
    }

    pub async fn delete(&self, _repo: &impl TodoRepo) -> Result<Option<Self>, DomainError> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct NewTodo {
    title: String,
    description: String,
    status: TodoStatus,
}

impl NewTodo {
    pub fn create(title: String, description: String) -> Result<NewTodo, DomainError> {
        Self::with_status(title, description, TodoStatus::Active)
    }

    pub fn with_status(
        title: String,
        description: String,
        status: TodoStatus,
    ) -> Result<NewTodo, DomainError> {
        Ok(NewTodo {
            title,
            description,
            status,
        })
    }
}

impl NewTodo {
    /// Get a reference to the new todo's text.
    pub fn title(&self) -> &String {
        &self.title
    }

    /// Get a reference to the new todo's status.
    pub fn status(&self) -> &TodoStatus {
        &self.status
    }

    /// Get a reference to the new todo's description.
    pub fn description(&self) -> &String {
        &self.description
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TodoStatus {
    Active,
    Completed,
}

#[cfg(test)]
mod tests {

    use super::{DomainError, NewTodo};
    #[test]
    fn create_fail() {
        let result = NewTodo::create(String::default(), String::from("desc"));
        assert!(matches!(result, Err(DomainError::NewTodoError)));

        let result = NewTodo::create(String::from("title"), String::default());
        assert!(matches!(result, Err(DomainError::NewTodoError)))
    }
}
