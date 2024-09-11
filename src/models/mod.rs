use serde::Deserialize;

#[derive(Deserialize)]
pub struct Post {
    pub id: String,
    pub content: String,
    pub createdAt: String,
}