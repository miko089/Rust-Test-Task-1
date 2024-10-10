use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub posts: Vec<crate::models::BlogPost>,
}
