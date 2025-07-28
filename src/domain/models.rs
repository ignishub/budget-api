use std::{num::NonZeroI64, str::FromStr};

use sqlx::types::chrono::{self, Local};
use strum::EnumString;
use thiserror::Error;

#[derive(Debug, EnumString, Clone, strum_macros::Display)]
pub enum RecordType {
    Income,
    Outcome,
    Transfer,
}

const MAX_CATEGORY_NAME_LENGTH: usize = 100;

#[derive(Debug, Error)]
pub enum CategoryError {
    #[error("category name must not be empty or longer than 100 characters")]
    InvalidCategoryName,
}

#[derive(Clone, Debug, Default)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub budget: Option<i64>,
    pub parent_id: Option<i64>,
}

impl Category {
    pub fn new(
        name: String,
        budget: Option<i64>,
        parent_id: Option<i64>,
    ) -> Result<Self, CategoryError> {
        if name.chars().count() == 0 || name.chars().count() > MAX_CATEGORY_NAME_LENGTH {
            return Err(CategoryError::InvalidCategoryName);
        }

        Ok(Self {
            id: 0,
            name,
            budget,
            parent_id,
        })
    }
}

#[derive(Debug, Error)]
pub enum RecordError {
    #[error("amount cannot be equal or less than zero")]
    AmountCannotBeLessOrEqualToZero,
    #[error("invalid record type \"{0}\"")]
    InvalidRecordType(String),
}

#[derive(Debug, Clone)]
pub struct Record {
    pub id: i64,
    pub record_type: RecordType,
    pub amount: NonZeroI64,
    pub description: Option<String>,
    pub category: Option<Category>,
    pub created_at: chrono::DateTime<Local>,
    pub updated_at: chrono::DateTime<Local>,
}

impl Record {
    pub fn new(
        record_type: String,
        amount: i64,
        category: Option<Category>,
        description: Option<String>,
    ) -> Result<Self, RecordError> {
        let transaction_type = RecordType::from_str(&record_type)
            .map_err(|_| RecordError::InvalidRecordType(record_type))?;

        let amount = NonZeroI64::try_from(amount)
            .map_err(|_| RecordError::AmountCannotBeLessOrEqualToZero)?;

        Ok(Self {
            id: 0,
            record_type: transaction_type,
            amount,
            category,
            description,
            created_at: Local::now(),
            updated_at: Local::now(),
        })
    }
}
