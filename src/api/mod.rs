use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Post {
    pub id: String,
    pub content: String,
}

pub async fn fetch_posts(client: &Client, url: &str) -> Result<Vec<Post>, reqwest::Error> {
    let response = client.get(url).send().await?;
    let posts = response.json::<Vec<Post>>().await?;
    Ok(posts)
}