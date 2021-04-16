pub static CREATE_TODO: &str = r#"
    mutation createTodo($title: String, $description: String){
        createTodo(title : $title, description : $description) {
            id
            title
            description
            status
        }
    }
"#;

pub static CHANGE_STATUS_MUTATION: &str = r#"
mutation changeStatus($id: String, $status: String){
    changeStatus(todoId : $id, status : $status) {
        id
        title
        description
        status
    }
}
"#;

pub static DELETE_MUTATION: &str = r#"
mutation deleteTodo($id: String){
    deleteTodo(todoId : $id) {
        id
        title
        description
        status
    }
}
"#;

pub static GET_TODO: &str = r#"
    query todo($id: String){
        todo(todoId : $id) {
            id
            title
            description
            status
        }
    }
"#;

pub static GET_TODOS: &str = r#"
    query {
        todos {
            id
            title
            description
            status
        }
    }
"#;
