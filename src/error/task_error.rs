use actix_web::{
    http:: {header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

use derive_more::Display;
#[derive(Debug, Display)]
pub enum TaskError {
    NoTasksFound = 0,
    TaskCretionError = 1,
    NoTaskFoundWithId = 2,
}

impl ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody>{
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::NoTasksFound => StatusCode::NOT_FOUND,
            TaskError::TaskCretionError => StatusCode::INTERNAL_SERVER_ERROR,
            TaskError::NoTaskFoundWithId => StatusCode::NOT_FOUND,
        }
    }
}