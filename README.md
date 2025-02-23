# Actix CRUD Application

![Rust](https://img.shields.io/badge/Rust-1.77.1-blue)
![Actix](https://img.shields.io/badge/Actix-4.5.1-blue)
![SurrealDB](https://img.shields.io/badge/SurrealDB-1.4.0-blue)

## Description

This project is a CRUD (Create, Read, Update, Delete) application built with Rust and the Actix framework. The application uses SurrealDB as its database and provides JWT authentication for user management. It is an excellent demonstration of how to build a robust and secure RESTful API with Rust.

## Features

- **Task Management**: Create, read, update, and delete tasks.
- **JWT Authentication**: User registration and authentication with JWT tokens.
- **Data Validation**: Input data validation using `validator`.
- **SurrealDB Integration**: Interaction with SurrealDB.
- **Logging Middleware**: Custom middleware for logging requests and responses.

## Project Structure

```plaintext
.
├── .gitignore
├── .vscode/
│   └── settings.json
├── Cargo.lock
├── Cargo.toml
├── microservices/
│   ├── notification_service/
│   └── report_service/
├── mydatabase.db/
│   ├── 000031.sst
│   ├── 000034.sst
│   ├── 000039.sst
│   ├── 000040.log
│   ├── CURRENT
│   ├── IDENTITY
│   ├── LOCK
│   ├── LOG
│   ├── MANIFEST-000041
│   └── OPTIONS-000038
├── src/
│   ├── application/
│   ├── config/
│   ├── error/
│   ├── infrastructure/
│   ├── interfaces/
│   ├── main.rs
│   ├── models/
│   └── tests/
└── target/
```

## Installation

1. **Clone the repository**:
    ```sh
    git clone https://github.com/your-username/actix-crud.git
    cd actix-crud
    ```

2. **Set up environment variables**:
    Create a `.env` file in the root of the project with the following content:
    ```env
    DATABASE_URL=your_database_url
    SECRET_KEY=your_secret_key
    ```

3. **Install dependencies**:
    ```sh
    cargo build
    ```

4. **Run the application**:
    ```sh
    cargo run
    ```

## Usage

### Endpoints

- **GET /api/tasks**: Retrieve all tasks.
- **POST /api/tasks**: Create a new task.
- **PATCH /api/tasks/{uuid}**: Update an existing task.
- **POST /api/register**: Register a new user.
- **POST /api/login**: Log in and obtain a JWT token.

### Example Request

**Create a new task**:
```sh
curl -X POST http://127.0.0.1:8080/api/tasks \
-H "Content-Type: application/json" \
-d '{"task_name": "New Task"}'
```

## Contributing

Contributions are welcome! If you would like to contribute, please follow these steps:

1. Fork the project.
2. Create a new branch (`git checkout -b feature/new-feature`).
3. Make your changes and commit them (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature/new-feature`).
5. Open a Pull Request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

---

Thank you for visiting this project! If you have any questions or suggestions, feel free to open an issue or contact me.
