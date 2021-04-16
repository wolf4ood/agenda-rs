use agenda_domain::{
    errors::DomainError,
    todos::{NewTodo, TodoRepo, TodoStatus},
};

use fake::{Fake, Faker};

pub async fn create_todo_ok(repo: impl TodoRepo) {
    let input = mock_todo();
    let todo = repo
        .create(input.clone())
        .await
        .expect("It should return ok");

    let todo = repo
        .get(todo.id())
        .await
        .expect("It should return ok")
        .expect("It should not be empty");

    assert_eq!(todo.title(), input.title());
    assert_eq!(todo.status(), input.status());
}

pub async fn delete_todo_ok(repo: impl TodoRepo) {
    let input = mock_todo();
    let todo = repo
        .create(input.clone())
        .await
        .expect("It should return ok");

    let todo = todo
        .delete(&repo)
        .await
        .expect("It should delete the todo")
        .expect("It should not be empty");

    assert_eq!(todo.title(), input.title());
    assert_eq!(todo.status(), input.status());
}

pub async fn change_status_ok(repo: impl TodoRepo) {
    let input = mock_todo();
    let todo = repo
        .create(input.clone())
        .await
        .expect("It should return ok");

    let todo = todo
        .mark_completed(&repo)
        .await
        .expect("It should return ok");

    assert_eq!(todo.title(), input.title());

    assert_eq!(input.status(), &TodoStatus::Completed);
}

#[allow(dead_code)]
pub async fn change_status_fail(repo: impl TodoRepo) {
    let input = mock_todo();
    let todo = repo
        .create(input.clone())
        .await
        .expect("It should return ok");

    let todo = todo
        .mark_completed(&repo)
        .await
        .expect("It should return ok");

    assert_eq!(todo.title(), input.title());

    assert_eq!(input.status(), &TodoStatus::Completed);

    let result = todo.change_status(TodoStatus::Completed, &repo).await;

    assert!(matches!(result, Err(DomainError::ChangeStatusError)))
}

pub async fn list_todos(repo: impl TodoRepo) {
    let input = mock_todo();
    let todo = repo
        .create(input.clone())
        .await
        .expect("It should return ok");

    let todos = repo.all().await.expect("It should return ok");

    assert_eq!(todos.len(), 1);

    assert!(todos.iter().find(|t| t.id() == todo.id()).is_some())
}

pub fn mock_todo() -> NewTodo {
    NewTodo::create(Faker.fake::<String>(), Faker.fake::<String>()).unwrap()
}
