use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub namespace: String,
    pub db_name: String,
}

impl Database {
    // Initialize the database connection
    pub async fn init() -> Result<Self, Error> {
        println!("Starting connection to SurrealDB...");
        let client = Surreal::new::<Ws>("127.0.0.1:8002").await?;
        println!("Connection established. Signing in...");
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        println!("Signed in. Selecting namespace and database...");
        client.use_ns("surreal").use_db("task").await?;
        println!("Namespace and database selected.");
        Ok(Database {
            client,
            namespace: String::from("surreal"),
            db_name: String::from("task"),
        })
    }
}