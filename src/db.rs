use sqlx::{FromRow, PgPool};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: i32,
    pub text: String,
    pub mark: bool,
    pub parent_id: Option<i32>,
}

pub async fn init_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(database_url).await
}

pub async fn create_item(
    pool: &PgPool,
    text: &str,
    parent_id: Option<i32>,
) -> Result<TodoItem, sqlx::Error> {
    sqlx::query_as::<_, TodoItem>(
        "INSERT INTO todo_items (text, mark, parent_id) VALUES ($1, false, $2) RETURNING id, text, mark, parent_id",
    )
    .bind(text)
    .bind(parent_id)
    .fetch_one(pool)
    .await
}

pub async fn mark_item(pool: &PgPool, id: i32, mark: bool) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE todo_items SET mark = $1 WHERE id = $2")
        .bind(mark)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_item(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM todo_items WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn list_items(pool: &PgPool) -> Result<Vec<TodoItem>, sqlx::Error> {
    sqlx::query_as::<_, TodoItem>("SELECT id, text, mark, parent_id FROM todo_items ORDER BY id")
        .fetch_all(pool)
        .await
}
