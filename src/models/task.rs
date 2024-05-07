use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct AddTaskRequest {
    #[validate(length(min=1, message="task name required"))]
    pub task_name: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdateTaskUrl {
    pub uuid: String,
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct Task {
    pub uuid: String,
    pub task_name: String,
}

impl Task {
    pub fn new( uuid: String, task_name: String) -> Task{
        Task {uuid, task_name}
    }
}