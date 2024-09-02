use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::auth::hash_password;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(username: String, password: String) -> Result<Self, bcrypt::BcryptError> {
        let hashed_password = hash_password(&password)?;
        Ok(User {
            id: None,
            username,
            password: hashed_password,
        })
    }
}