use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Result},
};
use serde::{Deserialize, Serialize};

use crate::{
    domain::models,
    service::{
        budget::BudgetService,
        records::{CreateRecordCmd, ListRecordsCmd, UpdateRecordCmd},
    },
};

type State = Extension<Arc<dyn BudgetService>>;

#[derive(Serialize)]
struct Record {
    id: i64,
    amount: i64,
    record_type: String,
    category_id: Option<i64>,
}

impl From<&models::Record> for Record {
    fn from(record: &models::Record) -> Self {
        Self {
            id: record.id,
            amount: record.amount.into(),
            record_type: record.record_type.to_string(),
            category_id: record.category.clone().map(|c| c.id),
        }
    }
}

#[derive(Deserialize)]
pub struct ListRecordsReq {
    limit: Option<u64>,
    offset: Option<u64>,
    category_id: Option<i64>,
}
#[derive(Serialize)]
pub struct ListRecordsResponse {
    data: Vec<Record>,
}

impl IntoResponse for ListRecordsResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub async fn list_records(
    Extension(svc): State,
    Query(req): Query<ListRecordsReq>,
) -> Result<ListRecordsResponse> {
    let result = svc
        .list_records(ListRecordsCmd {
            limit: req.limit,
            offset: req.offset,
            category_id: req.category_id,
        })
        .await?;

    Ok(ListRecordsResponse {
        data: result.iter().map(Record::from).collect(),
    })
}

#[derive(Deserialize)]
pub struct CreateRecordRequest {
    account_id: i64,
    transaction_type: String,
    amount: i64,
    category: Option<i64>,
    description: Option<String>,
}

#[derive(Serialize)]
pub struct CreateRecordResponse {
    data: Record,
}

impl IntoResponse for CreateRecordResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

pub async fn create_record(
    Extension(svc): State,
    Json(req): Json<CreateRecordRequest>,
) -> Result<CreateRecordResponse> {
    let result = svc
        .create_record(CreateRecordCmd {
            account_id: req.account_id,
            transaction_type: req.transaction_type,
            amount: req.amount,
            category: req.category,
            description: req.description,
        })
        .await?;
    Ok(CreateRecordResponse {
        data: Record::from(&result),
    })
}

#[derive(Deserialize)]
pub struct UpdateRecordRequest {
    amount: i64,
    description: Option<String>,
    category_id: Option<i64>,
}

#[derive(Serialize)]
pub struct UpdateRecordResponse {
    data: Record,
}

impl IntoResponse for UpdateRecordResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub async fn update_record(
    Path(id): Path<i64>,
    Extension(svc): State,
    Json(req): Json<UpdateRecordRequest>,
) -> Result<UpdateRecordResponse> {
    let result = svc
        .update_record(UpdateRecordCmd {
            id,
            amount: req.amount,
            description: req.description,
            category_id: req.category_id,
        })
        .await?;

    Ok(UpdateRecordResponse {
        data: Record::from(&result),
    })
}
pub async fn delete_record(Path(id): Path<i64>, Extension(svc): State) -> impl IntoResponse {
    let result = svc.delete_record(id).await;

    match result {
        Ok(()) => (StatusCode::OK).into_response(),
        Err(e) => e.into_response(),
    }
}
