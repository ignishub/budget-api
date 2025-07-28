use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::domain::errors::BudgetServiceError;

#[derive(Serialize)]
pub struct JsonError {
    code: String,
    message: String,
}

impl JsonError {
    pub fn response(httpcode: StatusCode, code: String, message: String) -> Response {
        (httpcode, Json(Self { code, message })).into_response()
    }
}

impl IntoResponse for BudgetServiceError {
    fn into_response(self) -> Response {
        match &self {
            Self::RecordValidationError(e) => JsonError::response(
                StatusCode::BAD_REQUEST,
                "RecordValidationError".into(),
                e.to_string(),
            ),
            Self::CategoryValidationError(e) => JsonError::response(
                StatusCode::BAD_REQUEST,
                "CategoryValidationError".into(),
                e.to_string(),
            ),
            Self::AccountValidationError(e) => JsonError::response(
                StatusCode::BAD_REQUEST,
                "AccountValidationError".into(),
                e.to_string(),
            ),
            Self::EntityNotFoundError(_) => JsonError::response(
                StatusCode::NOT_FOUND,
                "EntityNotFoundError".into(),
                self.to_string(),
            ),

            Self::DatabaseError(e) => {
                dbg!(e);
                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
        }
    }
}
