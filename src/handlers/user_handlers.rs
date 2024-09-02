use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::models::User;
use crate::auth::{verify_password, generate_token};
use crate::db::{Database, user_data_trait::UserDataTrait};

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
}

pub async fn register(user_data: web::Json<RegisterRequest>, db: web::Data<Database>) -> impl Responder {
    let user = match User::new(user_data.username.clone(), user_data.password.clone()) {
        Ok(user) => user,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match Database::add_user(&db, user).await {
        Some(_) => HttpResponse::Ok().json("User registered successfully"),
        None => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn login(user_data: web::Json<LoginRequest>, db: web::Data<Database>) -> impl Responder {
    let user = match Database::find_user_by_username(&db, &user_data.username).await {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().finish(),
    };

    if !verify_password(&user_data.password, &user.password) {
        return HttpResponse::Unauthorized().finish();
    }

    let token = match user.id {
        Some(ref id) => match generate_token(id) {
            Ok(token) => token,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        },
        None => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Ok().json(AuthResponse { token })
}