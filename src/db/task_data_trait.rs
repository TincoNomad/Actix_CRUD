use crate::models::Task;
use crate::db::Database;
use surrealdb::Error;
use actix_web::web::Data;
use async_trait::async_trait;

#[async_trait]
pub trait TaskDataTrait {
    async fn get_all_task(db: &Data<Database>) -> Option<Vec<Task>>;
    async fn add_task(db: &Data<Database>, new_task: Task) -> Option<Task>;
    async fn update_task(db: &Data<Database>, uuid: String) -> Option<Task>;
}

#[async_trait]
impl TaskDataTrait for Database {

    async fn get_all_task(db: &Data<Database>) -> Option<Vec<Task>> {
        let result = db.client.select("task").await;
        match result {
            Ok(all_task) => Some(all_task),
            Err(_) => None,
        }
    }

    async fn add_task(db: &Data<Database>, new_task: Task) -> Option<Task> {
        let created_task = db
            .client
            .create(("task", new_task.uuid.clone()))
            .content(new_task)
            .await;
        match created_task {
            Ok(created) => created,
            Err(_) => None,
        }
    }

    async fn update_task(db: &Data<Database>, uuid: String) -> Option<Task> {
        let find_task: Result<Option<Task>, Error> = db.client.select(("task", &uuid)).await;

        match find_task {
            Ok(found) => {
                match  found {
                    Some(_found_task) => {
                        let updated_task: Result<Option<Task>, Error> = db
                            .client
                            .update(("task", &uuid))
                            .merge(Task {
                                uuid,
                                task_name: String::from("Completed")
                            })
                            .await;
                        match updated_task {
                            Ok(updated) => updated,
                            Err(_) => None,
                        }
                    },
                    None => None,
                }
                    
            },
            Err(_) => None,
        }
    }
}