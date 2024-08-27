use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::auth::hash_password;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(username: String, password: String) -> Result<Self, bcrypt::BcryptError> {
        let hashed_password = hash_password(&password)?;
        Ok(User {
            id: Uuid::new_v4(),
            username,
            password: hashed_password,
        })
    }
}