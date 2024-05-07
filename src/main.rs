use actix_web ::{get, post, patch, HttpServer, App, web::Data, web::Json, web::Path};
use crate::models::task::{AddTaskRequest, UpdateTaskUrl, Task};
use crate::db::{task_data_trait::TaskDataTrait, Database};
use validator::Validate;
use error::TaskError;
mod models;
mod error;
mod db;


#[get("/task")]
async fn get_task(db: Data<Database>) -> Result<Json<Vec<Task>>, TaskError> {
    let tasks = Database::get_all_task(&db).await;
    match tasks {
        Some(found_task) => Ok(Json(found_task)),
        None => Err(TaskError::NoTasksFound)
    }
}

#[post("/task")]
async fn add_task(body: Json<AddTaskRequest>, db: Data<Database>) -> Result<Json<Task>, TaskError> {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) =>{
            let task_name = body.task_name.clone();
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_task = Database::add_task(&db, Task::new(
                String::from(new_uuid),
                task_name,
            )).await;

            match new_task {

                Some(created) =>{
                    Ok(Json(created))
                },
                None => Err(TaskError::TaskCretionError),       
            }
        }
        Err(_) => Err(TaskError::TaskCretionError)
    }
}

#[patch("/updatetask/{uuid}")]
async fn update_task(
    update_task_url: Path<UpdateTaskUrl>,
    db: Data<Database>
) -> Result<Json<Task>, TaskError> {
    let uuid = update_task_url.into_inner().uuid;
    let update_result = Database::update_task(&db, uuid).await;

    match update_result {
        Some(updated_task) => Ok(Json(updated_task)),
        None => Err(TaskError::NoTaskFoundWithId),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = Database::init().await.expect("Error connecting to database");
    let db_data =Data::new(db);

    HttpServer::new(move || {
        App::new()
        .app_data(db_data.clone())
        .service(get_task)
        .service(add_task)
        .service(update_task)
    })
        .bind("127.0.0.1:8002")?
        .run()
        .await
}
