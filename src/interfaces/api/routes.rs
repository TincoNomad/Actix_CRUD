use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/tasks", web::get().to(crate::interfaces::api::task_handlers::get_task))
            .route("/tasks", web::post().to(crate::interfaces::api::task_handlers::add_task))
            .route("/tasks/{uuid}", web::patch().to(crate::interfaces::api::task_handlers::update_task))
            .route("/register", web::post().to(crate::interfaces::api::user_handlers::register))
            .route("/login", web::post().to(crate::interfaces::api::user_handlers::login))
    );
}
