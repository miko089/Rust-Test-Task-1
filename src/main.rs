use std::error::Error;
use std::env;

mod database;
mod models;
mod utils;
mod routes;
mod template;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let db_name = 
        format!(
            "db/{}",
            env::var("DB_NAME").unwrap_or("db.sqlite".to_string())
        );
    if !std::path::Path::new(&db_name).exists() {
        std::fs::File::create(&db_name)?;
    }
    let address = env::var("ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("3000".to_string());
    let db = database::init_database(
        &db_name, 5)
        .await?;
    let app = axum::Router::new();
    let app = routes::register_routers(app, db).await;
    let listener = 
        tokio::net::TcpListener::bind(format!("{address}:{port}"))
        .await?;
    axum::serve(listener, app).await?;
    Ok(())
}
