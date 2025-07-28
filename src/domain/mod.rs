use crate::domain::errors::BudgetServiceError;

pub mod errors;
pub mod models;

pub type Result<T, E = BudgetServiceError> = core::result::Result<T, E>;
