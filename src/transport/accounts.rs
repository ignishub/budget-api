use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Result},
};

use serde::{Deserialize, Serialize};

use crate::{
    domain::models,
    service::{
        accounts::{CreateAccountCmd, UpdateAccountCmd},
        budget::BudgetService,
    },
};

type State = Extension<Arc<dyn BudgetService>>;

#[derive(Serialize)]
pub struct Account {
    id: i64,
    name: String,
}

impl From<&models::Account> for Account {
    fn from(dto: &models::Account) -> Self {
        Self {
            id: dto.id,
            name: dto.name.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct ListAccountsResponse {
    data: Vec<Account>,
}

impl IntoResponse for ListAccountsResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl From<Vec<models::Account>> for ListAccountsResponse {
    fn from(value: Vec<models::Account>) -> Self {
        let data = value.iter().map(Account::from).collect();
        Self { data }
    }
}

pub async fn list_accounts(Extension(svc): State) -> Result<ListAccountsResponse> {
    let result = svc.list_accounts().await?;
    Ok(result.into())
}

#[derive(Deserialize)]
pub struct CreateAccountRequest {
    name: String,
    account_type: String,
    initial_balance: i64,
}

#[derive(Serialize)]
pub struct CreateAccountResponse {
    data: Account,
}

impl IntoResponse for CreateAccountResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

pub async fn create_account(
    Extension(svc): State,
    Json(req): Json<CreateAccountRequest>,
) -> Result<CreateAccountResponse> {
    let data = svc
        .create_account(CreateAccountCmd {
            name: req.name,
            account_type: req.account_type,
            initial_balance: req.initial_balance,
        })
        .await?;

    Ok(CreateAccountResponse {
        data: Account::from(&data),
    })
}

#[derive(Deserialize)]
pub struct UpdateAccountRequest {
    name: String,
}

#[derive(Serialize)]
pub struct UpdateAccountResponse {
    data: Account,
}

impl IntoResponse for UpdateAccountResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub async fn update_account(
    Path(id): Path<i64>,
    Extension(svc): State,
    Json(req): Json<UpdateAccountRequest>,
) -> Result<UpdateAccountResponse> {
    let data = svc
        .update_account(UpdateAccountCmd { id, name: req.name })
        .await?;

    Ok(UpdateAccountResponse {
        data: Account::from(&data),
    })
}

pub async fn delete_account(Path(id): Path<i64>, Extension(svc): State) -> impl IntoResponse {
    let result = svc.delete_account(id).await;
    match result {
        Ok(()) => (StatusCode::OK).into_response(),
        Err(e) => e.into_response(),
    }
}
