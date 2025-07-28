use budget_api::{
    repository::{SqliteBudgetRepo, migrations},
    service::budget::BudgetServiceImpl,
    transport::router,
};
use sqlx::SqlitePool;

#[tokio::main]
async fn main() {
    // let options = SqliteConnectOptions::new()
    // .filename("budget.db")
    // .create_if_missing(true);

    // let pool = SqlitePool::connect_with(options)
    let pool = SqlitePool::connect("file::memory:?cache=shared")
        .await
        .expect("cannot connect to sqlite");

    migrations::sqlite_migrate(&pool).await;

    let repo = SqliteBudgetRepo::new(pool);
    let svc = BudgetServiceImpl::new(repo);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000")
        .await
        .expect("cannot bind to addr");

    axum::serve(listener, router::new(svc)).await.unwrap();
}
