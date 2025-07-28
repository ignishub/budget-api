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
        budget::BudgetService,
        categories::{CreateCategoryCmd, UpdateCategoryCmd},
    },
};

type State = Extension<Arc<dyn BudgetService>>;

#[derive(Serialize)]
pub struct Category {
    id: i64,
    name: String,
    budget: Option<i64>,
    parent_id: Option<i64>,
}

impl From<&models::Category> for Category {
    fn from(dto: &models::Category) -> Self {
        Self {
            id: dto.id,
            name: dto.name.clone(),
            budget: dto.budget,
            parent_id: dto.parent_id,
        }
    }
}

#[derive(Serialize)]
pub struct ListCategoryResponse {
    data: Vec<Category>,
}

impl IntoResponse for ListCategoryResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub async fn list_categories(Extension(svc): State) -> Result<ListCategoryResponse> {
    let result = svc.list_categories().await?;
    let result = result.iter().map(Category::from).collect();

    Ok(ListCategoryResponse { data: result })
}

#[derive(Deserialize)]
pub struct CreateCategoryRequest {
    name: String,
    budget: Option<i64>,
    parent_id: Option<i64>,
}

#[derive(Serialize)]
pub struct CreateCategoryResponse {
    data: Category,
}

impl IntoResponse for CreateCategoryResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

pub async fn create_category(
    Extension(svc): State,
    Json(req): Json<CreateCategoryRequest>,
) -> Result<CreateCategoryResponse> {
    let result = svc
        .create_category(CreateCategoryCmd {
            name: req.name,
            budget: req.budget,
            parent_id: req.parent_id,
        })
        .await?;

    Ok(CreateCategoryResponse {
        data: Category::from(&result),
    })
}

#[derive(Deserialize)]
pub struct UpdateCategoryRequest {
    name: String,
    budget: Option<i64>,
}

#[derive(Serialize)]
pub struct UpdateCategoryResponse {
    data: Category,
}

impl IntoResponse for UpdateCategoryResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub async fn update_category(
    Path(id): Path<i64>,
    Extension(svc): State,
    Json(req): Json<UpdateCategoryRequest>,
) -> Result<UpdateCategoryResponse> {
    let result = svc
        .update_category(UpdateCategoryCmd {
            id,
            name: req.name,
            budget: req.budget,
        })
        .await?;

    Ok(UpdateCategoryResponse {
        data: Category::from(&result),
    })
}

pub async fn delete_category(Path(id): Path<i64>, Extension(svc): State) -> impl IntoResponse {
    let result = svc.delete_category(id).await;
    match result {
        Ok(()) => (StatusCode::OK).into_response(),
        Err(e) => e.into_response(),
    }
}
