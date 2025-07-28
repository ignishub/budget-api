use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("database error {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("{entityname} not found")]
    NotFound { entityname: String },
}
