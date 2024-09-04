use actix_web::web::Data;
use crate::infrastructure::database::surrealdb::Database;
use crate::models::traits::task_data_trait::TaskDataTrait;
use crate::models::traits::user_data_trait::UserDataTrait;

// UseCaseExecutor is a utility struct for executing use cases
pub struct DataTraitExecutor;

impl DataTraitExecutor {
    // Execute a task data trait
    // F: The function to execute
    // Fut: The future returned by F
    // R: The result type of the future
    pub async fn execute_task_data_trait<F, Fut, R>(db: &Data<Database>, f: F) -> R
    where
        F: FnOnce(&dyn TaskDataTrait) -> Fut,
        Fut: std::future::Future<Output = R>,
    {
        // Call the function f with a reference to the Database,
        // which implements TaskUseCases, and await the result
        f(db.as_ref()).await
    }

    // Execute a user data trait
    // F: The function to execute
    // Fut: The future returned by F
    // R: The result type of the future
    pub async fn execute_user_data_trait<F, Fut, R>(db: &Data<Database>, f: F) -> R
    where
        F: FnOnce(&dyn UserDataTrait) -> Fut,
        Fut: std::future::Future<Output = R>,
    {
        // Call the function f with a reference to the Database,
        // which implements UserUseCases, and await the result
        f(db.as_ref()).await
    }
}