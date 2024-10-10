
#[derive(sqlx::FromRow, Debug)]
pub struct Post {
    pub username: String,
    pub main_text: String,
    pub image: Option<Vec<u8>>,
    pub avatar: Option<Vec<u8>>,
    pub created_at: sqlx::types::time::PrimitiveDateTime
}

pub struct BlogPost {
    pub username: String,
    pub main_text: String,
    pub date: String,
    pub image: Option<String>,
    pub avatar: Option<String>,
}