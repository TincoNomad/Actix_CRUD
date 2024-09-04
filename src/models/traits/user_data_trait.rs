use crate::models::entities::user::User;
use crate::infrastructure::database::surrealdb::Database;
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
            .create("user")
            .content(&new_user)
            .await;
        
            match created_user {
                Ok(mut users) => {
                    if let Some(user) = users.pop() {
                        println!("Usuario creado con éxito.");
                        Some(user)
                    } else {
                        println!("No se pudo crear el usuario.");
                        None
                    }
                },
                Err(e) => {
                    println!("Error al crear usuario: {:?}", e);
                    None
                },
            }
    }

    async fn find_user_by_username(db: &Data<Database>, username: &str) -> Option<User> {
        println!("Buscando usuario por nombre de usuario: {}", username);
        let result = db
            .client
            .query("SELECT * FROM user WHERE username = $username")
            .bind(("username", username))
            .await;
        
        match result {
            Ok(mut response) => {
                match response.take::<Vec<User>>(0) {
                    Ok(users) => users.into_iter().next(),
                    Err(e) => {
                        println!("Error al deserializar usuario: {:?}", e);
                        None
                    }
                }
            },
            Err(e) => {
                println!("Error al buscar usuario: {:?}", e);
                None
            },
        }
    }
}