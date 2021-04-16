use thiserror::Error;
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("ChangeStatusError")]
    ChangeStatusError,
    #[error("NewTodoError")]
    NewTodoError,
    #[error("Store error")]
    Store(#[from] anyhow::Error),
}
