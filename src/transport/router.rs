use std::sync::Arc;

use axum::{
    Extension, Router,
    routing::{get, put},
};

use crate::{
    service::budget::BudgetService,
    transport::{
        accounts::{create_account, delete_account, list_accounts, update_account},
        categories::{create_category, delete_category, list_categories, update_category},
    },
};

use super::records::*;

pub fn new<T: BudgetService>(budget_svc: T) -> Router {
    let tx_svc = Arc::new(budget_svc) as Arc<dyn BudgetService>;
    Router::new()
        //
        .route("/accounts", get(list_accounts).post(create_account))
        .route("/accounts/{id}", put(update_account).delete(delete_account))
        //
        .route("/records", get(list_records).post(create_record))
        .route("/records/{id}", put(update_record).delete(delete_record))
        //
        .route("/categories", get(list_categories).post(create_category))
        .route(
            "/categories/{id}",
            put(update_category).delete(delete_category),
        )
        //
        .layer(Extension(tx_svc))
}
