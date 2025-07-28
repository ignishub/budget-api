use sqlx::{
    prelude::FromRow,
    types::chrono::{DateTime, Local},
};

use crate::domain::models::{self, Account, AccountType, Category, RecordType};

use std::str::FromStr;

#[derive(FromRow, Debug)]
pub struct ReturnedId {
    pub id: i64,
}

impl From<RecordType> for i64 {
    fn from(value: RecordType) -> Self {
        match value {
            RecordType::Income => 1,
            RecordType::Outcome => 2,
            RecordType::Transfer => 3,
        }
    }
}

#[derive(FromRow, Debug)]
pub struct AccountDTO {
    account_id: i64,
    name: String,
    account_type: String,
    current_balance: i64,
}

impl From<AccountDTO> for Account {
    fn from(dto: AccountDTO) -> Self {
        Self {
            id: dto.account_id,
            name: dto.name,
            account_type: AccountType::from_str(&dto.account_type)
                .expect("cannot convert account type from database"),
            balance: dto.current_balance,
        }
    }
}

#[derive(FromRow, Debug)]
pub struct RecordDTO {
    record_id: i64,
    account_id: i64,
    amount: i64,
    description: Option<String>,
    record_type: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

#[derive(FromRow, Debug)]
pub struct FullRecordDTO {
    #[sqlx(flatten)]
    record: RecordDTO,
    #[sqlx(flatten)]
    category: OptionalCategoryDTO,
}

impl From<FullRecordDTO> for models::Record {
    fn from(dto: FullRecordDTO) -> Self {
        Self {
            id: dto.record.record_id,
            account_id: dto.record.account_id,
            amount: dto
                .record
                .amount
                .try_into()
                .expect("cannot convert i64 to NonZeroI64"),
            description: dto.record.description,
            category: dto.category.into(),
            record_type: RecordType::from_str(&dto.record.record_type)
                .expect("cannot convert transaction type from db"),
            created_at: dto.record.created_at,
            updated_at: dto.record.updated_at,
        }
    }
}

#[derive(FromRow, Debug)]
pub struct CategoryDTO {
    category_id: i64,
    #[sqlx(default)]
    budget: Option<i64>,
    name: String,
    #[sqlx(default)]
    parent_id: Option<i64>,
}

impl From<CategoryDTO> for Category {
    fn from(dto: CategoryDTO) -> Self {
        Self {
            id: dto.category_id,
            budget: dto.budget,
            name: dto.name,
            parent_id: dto.parent_id,
        }
    }
}

#[derive(FromRow, Debug)]
pub struct OptionalCategoryDTO {
    #[sqlx(default)]
    category_id: Option<i64>,
    #[sqlx(default)]
    budget: Option<i64>,
    #[sqlx(default)]
    name: Option<String>,
    #[sqlx(default)]
    parent_id: Option<i64>,
}

impl From<OptionalCategoryDTO> for Option<Category> {
    fn from(dto: OptionalCategoryDTO) -> Self {
        let category_id = dto.category_id?;

        Some(Category {
            id: category_id,
            budget: dto.budget,
            name: dto.name.unwrap(),
            parent_id: dto.parent_id,
        })
    }
}
