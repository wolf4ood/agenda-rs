use agenda_memory::TodoRepoInMemory;

mod todo;

fn create_repo() -> TodoRepoInMemory {
    TodoRepoInMemory::new()
}
#[tokio::test]
async fn create_todo_ok() {
    let repo = create_repo();

    todo::create_todo_ok(repo).await;
}
#[tokio::test]
async fn create_todo_fail() {
    let repo = create_repo();

    todo::create_todo_fail(repo).await;
}

#[tokio::test]
async fn delete_todo_ok() {
    let repo = create_repo();

    todo::delete_todo_ok(repo).await;
}

#[tokio::test]
async fn change_status_ok() {
    let repo = create_repo();

    todo::change_status_ok(repo).await;
}

#[tokio::test]
async fn change_status_fail() {
    let repo = create_repo();

    todo::change_status_fail(repo).await;
}

#[tokio::test]
async fn list_todos() {
    let repo = create_repo();

    todo::list_todos(repo).await;
}
