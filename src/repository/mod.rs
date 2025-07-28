pub mod accounts;
pub mod categories;
mod dto;
pub mod errors;
pub mod migrations;
pub mod records;
mod test;

use sqlx::SqlitePool;

use crate::service::budget::BudgetRepository;

#[derive(Clone)]
pub struct SqliteBudgetRepo {
    pool: SqlitePool,
}

impl SqliteBudgetRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl BudgetRepository for SqliteBudgetRepo {}
