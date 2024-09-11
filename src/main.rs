use actix_web::{HttpServer, App, web, middleware::Logger};
use actix_crud::infrastructure::database::surrealdb::Database;
use actix_crud::interfaces::api::routes;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger with default settings
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    println!("Starting the application...");
    // Initialize the database connection
    let db = Database::init().await.expect("Error connecting to database");
    println!("Database connection established.");
    // Wrap the database connection in a web::Data for sharing across threads
    let db_data = web::Data::new(db);

    println!("Starting the HTTP server...");
    // Create and run the HTTP server
    HttpServer::new(move || {
        App::new()
            // Add logger middleware
            .wrap(Logger::default())
            // Share the database connection
            .app_data(db_data.clone())
            // Configure the API routes
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?  // Bind the server to localhost on port 8080
    .run()
    .await
}