use uuid::Uuid;

use crate::errors::DomainError;

use super::TodoRepo;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct TodoId(Uuid);

impl TodoId {
    pub fn generate() -> TodoId {
        Uuid::new_v4().into()
    }

    pub fn take(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for TodoId {
    fn from(id: Uuid) -> Self {
        TodoId(id)
    }
}

impl ToString for TodoId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: TodoId,
    pub title: String,
    pub description: String,
    pub status: TodoStatus,
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
        status: TodoStatus,
        repo: &impl TodoRepo,
    ) -> Result<Self, DomainError> {
        repo.update(&Todo { status, ..self }).await
    }

    pub async fn mark_completed(self, repo: &impl TodoRepo) -> Result<Self, DomainError> {
        self.change_status(TodoStatus::Completed, repo).await
    }

    pub async fn delete(&self, repo: &impl TodoRepo) -> Result<Option<Self>, DomainError> {
        repo.delete(self.id()).await
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
        if title.is_empty() {
            return Err(DomainError::NewTodoError);
        }
        if description.is_empty() {
            return Err(DomainError::NewTodoError);
        }
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
        assert!(
            matches!(result, Err(DomainError::NewTodoError)),
            "Title should not be empty"
        );

        let result = NewTodo::create(String::from("title"), String::default());
        assert!(
            matches!(result, Err(DomainError::NewTodoError)),
            "Description should not be empty"
        )
    }
}
