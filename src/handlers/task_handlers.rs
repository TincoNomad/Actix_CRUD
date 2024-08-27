use actix_web::{web, HttpResponse, Responder};
use crate::models::task::{AddTaskRequest, UpdateTaskUrl, Task};
use crate::db::{Database, task_data_trait::TaskDataTrait};
use crate::error::TaskError;
use validator::Validate;
use uuid::Uuid;

pub async fn get_task(db: web::Data<Database>) -> impl Responder {
    let tasks = Database::get_all_task(&db).await;
    match tasks {
        Some(found_task) => HttpResponse::Ok().json(found_task),
        None => HttpResponse::NotFound().json(TaskError::NoTasksFound)
    }
}

pub async fn add_task(body: web::Json<AddTaskRequest>, db: web::Data<Database>) -> impl Responder {
    if let Err(_) = body.validate() {
        return HttpResponse::BadRequest().json(TaskError::TaskCretionError);
    }

    let task_name = body.task_name.clone();
    let new_uuid = Uuid::new_v4().to_string();

    let new_task = Database::add_task(&db, Task::new(new_uuid, task_name)).await;

    match new_task {
        Some(created) => HttpResponse::Ok().json(created),
        None => HttpResponse::InternalServerError().json(TaskError::TaskCretionError),
    }
}

pub async fn update_task(
    update_task_url: web::Path<UpdateTaskUrl>,
    db: web::Data<Database>
) -> impl Responder {
    let uuid = update_task_url.into_inner().uuid;
    let update_result = Database::update_task(&db, uuid).await;

    match update_result {
        Some(updated_task) => HttpResponse::Ok().json(updated_task),
        None => HttpResponse::NotFound().json(TaskError::NoTaskFoundWithId),
    }
}