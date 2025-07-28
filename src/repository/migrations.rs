use sqlx::SqlitePool;

pub async fn sqlite_migrate(pool: &SqlitePool) {
    sqlx::migrate!()
        .run(pool)
        .await
        .expect("cannot connect to sqlite");
}
