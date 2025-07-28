use thiserror::Error;

use crate::domain::models;

#[derive(Debug, Error)]
pub enum BudgetServiceError {
    #[error("record validation error: {0}")]
    RecordValidationError(#[from] models::RecordError),
    #[error("cagegory validation error: {0}")]
    CategoryValidationError(#[from] models::CategoryError),
    #[error("account validation error: {0}")]
    AccountValidationError(#[from] models::AccountError),
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("{0} not found")]
    EntityNotFoundError(String),
}
