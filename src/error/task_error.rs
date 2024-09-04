use actix_web::{
    http:: {header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

use derive_more::Display;
use serde::Serialize;

#[derive(Debug, Display, Serialize)]
pub enum TaskError {
    NoTasksFound = 0,
    TaskCreationError = 1,
    NoTaskFoundWithId = 2,
}

impl ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody>{
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .json(self)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::NoTasksFound => StatusCode::NOT_FOUND,
            TaskError::TaskCreationError => StatusCode::INTERNAL_SERVER_ERROR,
            TaskError::NoTaskFoundWithId => StatusCode::NOT_FOUND,
        }
    }
}