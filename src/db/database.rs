use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        println!("Iniciando conexión a SurrealDB...");
        let client = Surreal::new::<Ws>("127.0.0.1:8002").await?;
        println!("Conexión establecida. Iniciando sesión...");
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        println!("Sesión iniciada. Seleccionando namespace y base de datos...");
        client.use_ns("surreal").use_db("task").await?;
        println!("Namespace y base de datos seleccionados.");
        Ok(Database {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("task"),
        })
    }
}
