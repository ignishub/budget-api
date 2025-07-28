#[cfg(test)]
pub async fn test_db() -> sqlx::SqlitePool {
    use crate::repository::migrations;

    let pool = sqlx::SqlitePool::connect("sqlite::memory:")
        .await
        .expect("cannot connect to sqlite");

    migrations::sqlite_migrate(&pool).await;

    pool
}
