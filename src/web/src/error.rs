use std::fmt::{Display, Formatter};

use actix_web::{body::Body, error, HttpResponse};

#[derive(Debug)]
pub struct MyError(anyhow::Error);

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        let response = HttpResponse::new(self.status_code());

        response.set_body(Body::from(serde_json::json!({
            "message": format!("{}", self)
        })))
    }
}

impl From<anyhow::Error> for MyError {
    fn from(err: anyhow::Error) -> Self {
        Self(err)
    }
}
