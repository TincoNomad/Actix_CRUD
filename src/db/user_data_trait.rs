use crate::models::User;
use crate::db::Database;
use surrealdb::Error;
use actix_web::web::Data;
use async_trait::async_trait;

#[async_trait]
pub trait UserDataTrait {
    async fn add_user(db: &Data<Database>, new_user: User) -> Option<User>;
    async fn find_user_by_username(db: &Data<Database>, username: &str) -> Option<User>;
}

#[async_trait]
impl UserDataTrait for Database {
    async fn add_user(db: &Data<Database>, new_user: User) -> Option<User> {
        println!("Intentando añadir un nuevo usuario...");
        let created_user = db
            .client
            .create(("user", new_user.id.to_string()))
            .content(new_user)
            .await;
        match created_user {
            Ok(created) => {
                println!("Usuario creado con éxito.");
                created
            },
            Err(e) => {
                println!("Error al crear usuario: {:?}", e);
                None
            },
        }
    }

    async fn find_user_by_username(db: &Data<Database>, username: &str) -> Option<User> {
        println!("Buscando usuario por nombre de usuario: {}", username);
        let result: Result<Option<User>, Error> = db
            .client
            .query("SELECT * FROM user WHERE username = $username")
            .bind(("username", username))
            .await
            .and_then(|mut response| response.take(0));
        
        match result {
            Ok(user) => {
                if user.is_some() {
                    println!("Usuario encontrado.");
                } else {
                    println!("Usuario no encontrado.");
                }
                user
            },
            Err(e) => {
                println!("Error al buscar usuario: {:?}", e);
                None
            },
        }
    }
}