use askama::Template;
use askama_axum::IntoResponse;
use axum::Extension;
use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::response::Html;
use sqlx::SqlitePool;
use crate::database::{get_posts, insert_post};
use crate::utils::{download_png, is_png};

pub async fn create_post(
    Extension(pool): Extension<SqlitePool>,
    mut form: Multipart
) -> Result<(), (StatusCode, String)> {
    let mut username = None;
    let mut main_text = None;
    let mut image = None;
    let mut avatar = None;
    while let Some(field) = form.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        match &name[..] {
            "username" => {
                let value = field.text().await.unwrap();
                username = Some(value);
            }
            "main_text" => {
                let value = field.text().await.unwrap();
                main_text = Some(value);
            }
            "image" => {
                let value = field.bytes().await.unwrap();
                if !value.is_empty() {
                    image = Some(value.to_vec());
                }
            }
            "avatar_url" => {
                let value = field.text().await.unwrap();
                if !value.is_empty() {
                    avatar = Some(value);
                }
            }
            _ => {}
        }
    }
    if let None = username {
        return Err((StatusCode::BAD_REQUEST, "username is required".to_string()));
    }
    let username = username.unwrap();
    if let None = main_text {
        return Err((StatusCode::BAD_REQUEST, "main_text is required".to_string()));
    }
    let main_text = main_text.unwrap();
    let image = image.map(|image| image.as_slice().to_vec());
    if let Some(img) = &image {
        if !is_png(img) {
            return Err((StatusCode::BAD_REQUEST, "image must be a png".to_string()));
        }
    }
    let mut avatar_downloaded = None;
    if let Some(url) = avatar {
        let avatar_bytes = download_png(&url).await;
        if let Err(e) = avatar_bytes {
            return Err((StatusCode::BAD_REQUEST, e.to_string()));
        }
        avatar_downloaded = Some(avatar_bytes.unwrap());
    }
    let avatar = 
        avatar_downloaded.map(|avatar| avatar.as_slice().to_vec());
    if let Err(e) = insert_post(&pool, username, main_text, image, avatar).await {
        Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    } else {
        Ok(())
    }
}

pub async fn home(
    Extension(pool): Extension<SqlitePool>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let posts = get_posts(&pool).await;
    if let Err(e) = posts {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    }
    let posts = posts.unwrap();
    let posts = posts.into_iter().map(|post| crate::models::BlogPost {
        username: post.username,
        main_text: post.main_text,
        date: post.created_at.to_string(),
        image: post.image.map(|x| base64::encode(&x)),
        avatar: post.avatar.map(|x| base64::encode(&x)),
    }).collect();
    let template = crate::template::HomeTemplate { posts };
    Ok(Html(template.render().unwrap()))
}