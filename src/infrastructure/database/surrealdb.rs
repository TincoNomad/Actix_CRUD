use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

// Database struct represents a connection to SurrealDB
#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,  // The SurrealDB client
    pub namespace: String,        // The namespace being used
    pub db_name: String,          // The name of the database
}

impl Database {
    // Initialize the database connection
    pub async fn init() -> Result<Self, Error> {
        println!("Starting connection to SurrealDB...");
        // Create a new SurrealDB client and connect to the server
        let client = Surreal::new::<Ws>("127.0.0.1:8002").await?;
        
        println!("Connection established. Signing in...");
        // Sign in to the database with root credentials
        // TODO: Use proper credential management instead of hardcoded values
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        
        println!("Signed in. Selecting namespace and database...");
        // Select the namespace and database to use
        client.use_ns("surreal").use_db("task").await?;
        
        println!("Namespace and database selected.");
        // Return a new Database instance
        Ok(Database {
            client,
            namespace: String::from("surreal"),
            db_name: String::from("task"),
        })
    }
}