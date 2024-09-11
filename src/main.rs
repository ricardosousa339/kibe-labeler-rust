use my_project::api::fetch_posts;
use my_project::db::save_posts;
use my_project::models::Post;
use reqwest::Client;
use tokio::runtime::Runtime;

fn main() {
    let runtime = Runtime::new().unwrap();
    let client = Client::new();
    let url = "https://api.bluesky.com/posts";

    let posts: Vec<Post> = runtime.block_on(fetch_posts(&client, url)).unwrap();

    runtime.block_on(save_posts(&posts)).unwrap();
}