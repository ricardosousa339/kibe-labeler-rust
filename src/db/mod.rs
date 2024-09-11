use crate::models::Post;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;

pub async fn save_posts(posts: &[Post]) -> Result<(), Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost/database")
        .await?;

    for post in posts {
        sqlx::query!(
            "INSERT INTO posts (id, content) VALUES ($1, $2)",
            post.id,
            post.content
        )
        .execute(&pool)
        .await?;
    }

    Ok(())
}