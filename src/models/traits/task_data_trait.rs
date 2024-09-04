use crate::models::entities::task::Task;
use crate::infrastructure::database::surrealdb::Database;
// use actix_web::web::Data;
use surrealdb::Error;
use async_trait::async_trait;

#[async_trait]
pub trait TaskDataTrait {
    async fn get_all_tasks(&self) -> Option<Vec<Task>>;
    async fn add_task(&self, new_task: Task) -> Option<Task>;
    async fn update_task(&self, uuid: String) -> Option<Task>;
}

#[async_trait]
impl TaskDataTrait for Database {
    async fn get_all_tasks(&self) -> Option<Vec<Task>> {
        println!("Intentando obtener todas las tareas...");
        let result = self.client.select("task").await;
        match result {
            Ok(all_tasks) => {
                println!("Tareas obtenidas con éxito.");
                Some(all_tasks)
            },
            Err(e) => {
                println!("Error al obtener las tareas: {:?}", e);
                None
            },
        }
    }

    async fn add_task(&self, new_task: Task) -> Option<Task> {
        println!("Intentando agregar una nueva tarea...");
        let created_task = self
            .client
            .create(("task", new_task.uuid.clone()))
            .content(new_task)
            .await;
        match created_task {
            Ok(created) => {
                println!("Tarea creada con éxito.");
                created
            },
            Err(e) => {
                println!("Error al crear la tarea: {:?}", e);
                None
            },
        }
    }

    async fn update_task(&self, uuid: String) -> Option<Task> {
        let find_task: Result<Option<Task>, Error> = self.client.select(("task", &uuid)).await;

        match find_task {
            Ok(found) => {
                match found {
                    Some(_found_task) => {
                        let updated_task: Result<Option<Task>, Error> = self
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
