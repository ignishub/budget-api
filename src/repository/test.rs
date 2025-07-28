#[cfg(test)]
use crate::repository::SqliteBudgetRepo;

#[cfg(test)]
pub async fn test_db(fixture: Option<&str>) -> SqliteBudgetRepo {
    use crate::repository::migrations;

    let pool = sqlx::SqlitePool::connect("sqlite::memory:")
        .await
        .expect("cannot connect to sqlite");

    migrations::sqlite_migrate(&pool).await;

    if let Some(fixture) = fixture {
        let mut conn = pool.acquire().await.unwrap();
        sqlx::query(fixture).execute(&mut *conn).await.unwrap();
    }

    SqliteBudgetRepo::new(pool)
}
