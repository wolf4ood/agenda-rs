use agenda_db::{drop_and_create_db, PgPool, TodoRepoDiesel};

mod todo;

fn create_repo(db_name: &str) -> TodoRepoDiesel {
    drop_and_create_db("postgres://agenda:agenda@localhost", db_name);

    TodoRepoDiesel::new(PgPool::new(
        format!("postgres://agenda:agenda@localhost/{}", db_name).as_str(),
    ))
}
#[tokio::test]
async fn create_todo_ok() {
    let repo = create_repo("create_todo_ok");

    todo::create_todo_ok(repo).await;
}
#[tokio::test]
async fn create_todo_fail() {
    let repo = create_repo("create_todo_fail");

    todo::create_todo_fail(repo).await;
}

#[tokio::test]
async fn delete_todo_ok() {
    let repo = create_repo("delete_todo_ok");

    todo::delete_todo_ok(repo).await;
}

#[tokio::test]
async fn change_status_ok() {
    let repo = create_repo("change_status_ok");

    todo::change_status_ok(repo).await;
}

#[tokio::test]
async fn list_todos() {
    let repo = create_repo("list_todos");

    todo::list_todos(repo).await;
}
