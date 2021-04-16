use actix_http::{Error, Request};
use actix_web::{
    dev::{Service, ServiceResponse},
    test, web, App,
};
use agenda_db::drop_and_create_db;
use agenda_web::{self, graphql, schema, ApplicationContext, PgPool, TodoRepoDiesel};
use serde_json::{json, Value};
use uuid::Uuid;

mod query;

use query::{CHANGE_STATUS_MUTATION, CREATE_TODO, DELETE_MUTATION, GET_TODO, GET_TODOS};
struct NewTodo {
    title: String,
    description: String,
}

#[actix_rt::test]
async fn integration_test_todo() {
    drop_and_create_db(
        "postgres://agenda:agenda@localhost",
        "integration_test_todo",
    );
    let pool = PgPool::new("postgres://agenda:agenda@localhost/integration_test_todo");
    let todos = TodoRepoDiesel::new(pool);
    let ctx = ApplicationContext::new(todos);

    let mut app = test::init_service(
        App::new()
            .data(schema(ctx))
            .route("/graphql", web::post().to(graphql)),
    )
    .await;

    let todo = NewTodo {
        title: String::from("title"),
        description: String::from("description"),
    };

    let id = create_and_check_todo(&mut app, &todo).await;

    fetch_and_check_todo(&mut app, id, &todo).await;
    fetch_and_check_todos(&mut app, id, &todo).await;

    mark_completed_and_check(&mut app, id, &todo).await;
    delete_and_check_todo(&mut app, id, &todo).await;
}

async fn create_and_check_todo(
    app: &mut impl Service<Request = Request, Response = ServiceResponse, Error = Error>,
    todo: &NewTodo,
) -> Uuid {
    let query = create_todo_query(todo);

    let req = test::TestRequest::post()
        .uri("/graphql")
        .set_json(&query)
        .to_request();
    let resp = test::call_service(app, req).await;

    assert!(resp.status().is_success());

    let mut value: Value = test::read_body_json(resp).await;

    let id = value
        .pointer_mut("/data/createTodo/id")
        .map(Value::take)
        .unwrap();

    assert_eq!(
        value,
        json!({
            "data" : {
                "createTodo" :  {
                    "id": null,
                    "title" : "title",
                    "description" : "description",
                    "status" : "ACTIVE"
                }
            }
        })
    );

    Uuid::parse_str(id.as_str().unwrap()).unwrap()
}

async fn fetch_and_check_todo(
    app: &mut impl Service<Request = Request, Response = ServiceResponse, Error = Error>,
    id: Uuid,
    NewTodo { title, description }: &NewTodo,
) {
    let query = get_todo_query(id);

    let req = test::TestRequest::post()
        .uri("/graphql")
        .set_json(&query)
        .to_request();
    let resp = test::call_service(app, req).await;

    assert!(resp.status().is_success());

    let value: Value = test::read_body_json(resp).await;

    assert_eq!(
        value,
        json!({
            "data" : {
                "todo" :  {
                    "id": id,
                    "title" : title,
                    "description" : description,
                    "status" : "ACTIVE"
                }
            }
        })
    );
}

async fn fetch_and_check_todos(
    app: &mut impl Service<Request = Request, Response = ServiceResponse, Error = Error>,
    id: Uuid,
    NewTodo { title, description }: &NewTodo,
) {
    let query = get_todos_query();

    let req = test::TestRequest::post()
        .uri("/graphql")
        .set_json(&query)
        .to_request();
    let resp = test::call_service(app, req).await;

    assert!(resp.status().is_success());

    let value: Value = test::read_body_json(resp).await;

    assert_eq!(
        value,
        json!({
            "data" : {
                "todos" :  [{
                    "id": id,
                    "title" : title,
                    "description" : description,
                    "status" : "ACTIVE"
                }]
            }
        })
    );
}

async fn mark_completed_and_check(
    app: &mut impl Service<Request = Request, Response = ServiceResponse, Error = Error>,
    id: Uuid,
    NewTodo { title, description }: &NewTodo,
) {
    let query = change_status_mutation(id, "COMPLETED");

    let req = test::TestRequest::post()
        .uri("/graphql")
        .set_json(&query)
        .to_request();
    let resp = test::call_service(app, req).await;

    assert!(resp.status().is_success());

    let value: Value = test::read_body_json(resp).await;

    assert_eq!(
        value,
        json!({
            "data" : {
                "changeStatus" :  {
                    "id": id,
                    "title" : title,
                    "description" : description,
                    "status" : "COMPLETED"
                }
            }
        })
    );
}

async fn delete_and_check_todo(
    app: &mut impl Service<Request = Request, Response = ServiceResponse, Error = Error>,
    id: Uuid,
    NewTodo { title, description }: &NewTodo,
) {
    let query = delete_mutation(id);

    let req = test::TestRequest::post()
        .uri("/graphql")
        .set_json(&query)
        .to_request();
    let resp = test::call_service(app, req).await;

    assert!(resp.status().is_success());

    let value: Value = test::read_body_json(resp).await;

    assert_eq!(
        value,
        json!({
            "data" : {
                "deleteTodo" :  {
                    "id": id,
                    "title" : title,
                    "description" : description,
                    "status" : "COMPLETED"
                }
            }
        })
    );
}

fn create_todo_query(NewTodo { title, description }: &NewTodo) -> Value {
    q(
        CREATE_TODO,
        json!({
            "title": title,
            "description" : description
        }),
    )
}

fn get_todo_query(id: Uuid) -> Value {
    q(
        GET_TODO,
        json!({
            "id": id,
        }),
    )
}

fn get_todos_query() -> Value {
    q(GET_TODOS, json!({}))
}

fn change_status_mutation(id: Uuid, status: &str) -> Value {
    q(
        CHANGE_STATUS_MUTATION,
        json!({ "id" : id, "status": status }),
    )
}

fn delete_mutation(id: Uuid) -> Value {
    q(DELETE_MUTATION, json!({ "id": id }))
}

fn q(input: &str, params: Value) -> Value {
    json!({ "query": input, "variables" : params })
}
