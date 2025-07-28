use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::domain::errors::BudgetServiceError;

impl IntoResponse for BudgetServiceError {
    fn into_response(self) -> Response {
        match self {
            Self::RecordValidationError(e) => {
                (StatusCode::BAD_REQUEST, e.to_string()).into_response()
            }
            Self::CategoryValidationError(e) => {
                (StatusCode::BAD_REQUEST, e.to_string()).into_response()
            }
            Self::DatabaseError(e) => {
                dbg!(e);
                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
        }
    }
}
