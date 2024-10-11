# Rust-Test-Task-1
## Crates that were used
- axum
- tokio
- sqlx
- askama
- reqwest
- base64
## How to use docker to run a project
1. Clone the repository
2. Run the following command in the root directory of the project
```bash
docker-compose up
```
3. Open the browser and go to the following address
```bash
http://localhost:3002/home
```

## What you can change
- You can change the port, but you must do it both in 
docker-compose.yaml and Dockerfile
- You can change the database path in the Dockerfile, but
it will appear in "db" folder in the root directory of the project.
Database is persistent, so you can restart the container and the data will be saved.

## Task
I won't publish the task itself, because it can be used by other students.
