use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Executor;
use std::error::Error;
use crate::models;

pub async fn init_database(path: &str, connections: u32) -> Result<SqlitePool, Box<dyn Error>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(connections)
        .connect(format!("sqlite:{}", path).as_str())
        .await?;
    pool.execute(
        r#"CREATE TABLE IF NOT EXISTS posts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                main_text TEXT NOT NULL,
                image BLOB,
                avatar BLOB,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
        "#,
    ).await?;
    Ok(pool)
}

pub async fn insert_post(
    pool: &SqlitePool,
    username: String,
    main_text: String,
    image: Option<Vec<u8>>,
    avatar: Option<Vec<u8>>,
) -> Result<(), Box<dyn Error>> {
    sqlx::query("INSERT INTO posts (username, main_text, image, avatar) VALUES (?, ?, ?, ?)")
        .bind(username)
        .bind(main_text)
        .bind(image)
        .bind(avatar)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_posts(pool: &SqlitePool) -> Result<Vec<models::Post>, Box<dyn Error>> {
    let posts = sqlx::query_as::<_, models::Post>(
        "SELECT * FROM posts ORDER BY created_at DESC"
        ).fetch_all(pool)
        .await?;
    Ok(posts)
}