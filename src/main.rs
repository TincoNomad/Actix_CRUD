use actix_web::{HttpServer, App, web, middleware::Logger};
use actix_crud::infrastructure::database::surrealdb::Database;
use actix_crud::interfaces::api::routes;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    println!("Iniciando la aplicación...");
    let db = Database::init().await.expect("Error connecting to database");
    println!("Conexión a la base de datos establecida.");
    let db_data = web::Data::new(db);

    println!("Iniciando el servidor HTTP...");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_data.clone())
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}