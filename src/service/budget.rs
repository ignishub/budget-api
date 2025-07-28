use std::num::NonZeroI64;

use async_trait::async_trait;
use sqlx::types::chrono::Local;

use crate::domain::{
    Result,
    errors::BudgetServiceError,
    models::{Category, Record, RecordError},
};

#[async_trait]
pub trait RecordRepository: Clone + Send + Sync + 'static {
    async fn list_records(&self, req: ListRecordsCmd) -> Result<Vec<Record>>;
    async fn create_record(&self, transaction: Record) -> Result<i64>;
    async fn get_record_by_id(&self, id: i64) -> Result<Record>;
    async fn update_record(&self, record: Record) -> Result<()>;
    async fn delete_record(&self, id: i64) -> Result<()>;
}

#[async_trait]
pub trait CategoryRepository: Clone + Send + Sync + 'static {
    async fn list_categories(&self) -> Result<Vec<Category>>;
    async fn create_category(&self, category: Category) -> Result<i64>;
    async fn get_category_by_id(&self, id: i64) -> Result<Category>;
    async fn update_category(&self, category: Category) -> Result<()>;
    async fn delete_category(&self, id: i64) -> Result<()>;
}

pub trait BudgetRepository: RecordRepository + CategoryRepository {}

#[derive(Debug)]
pub struct CreateRecordCmd {
    pub transaction_type: String,
    pub amount: i64,
    pub category: Option<i64>,
}

#[derive(Debug)]
pub struct ListRecordsCmd {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub category_id: Option<i64>,
}

pub struct CreateCategoryCmd {
    pub name: String,
    pub budget: Option<i64>,
    pub parent_id: Option<i64>,
}

pub struct UpdateRecordCmd {
    pub id: i64,
    pub amount: i64,
    pub description: Option<String>,
    pub category_id: Option<i64>,
}

pub struct UpdateCategoryCmd {
    pub id: i64,
    pub name: String,
    pub budget: Option<i64>,
}

#[async_trait]
pub trait BudgetService: Send + Sync + 'static {
    async fn list_records(&self, cmd: ListRecordsCmd) -> Result<Vec<Record>>;
    async fn create_record(&self, cmd: CreateRecordCmd) -> Result<Record>;
    async fn update_record(&self, cmd: UpdateRecordCmd) -> Result<Record>;
    async fn delete_record(&self, id: i64) -> Result<()>;

    async fn list_categories(&self) -> Result<Vec<Category>>;
    async fn create_category(&self, cmd: CreateCategoryCmd) -> Result<Category>;
    async fn update_category(&self, cmd: UpdateCategoryCmd) -> Result<Category>;
    async fn delete_category(&self, id: i64) -> Result<()>;
}

#[derive(Clone)]
pub struct BudgetServiceImpl<T: BudgetRepository> {
    repo: T,
}

impl<T: BudgetRepository> BudgetServiceImpl<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<T: BudgetRepository> BudgetService for BudgetServiceImpl<T> {
    async fn list_records(&self, cmd: ListRecordsCmd) -> Result<Vec<Record>> {
        Ok(self.repo.list_records(cmd).await?)
    }

    async fn create_record(&self, cmd: CreateRecordCmd) -> Result<Record> {
        let mut category: Option<Category> = None;
        if let Some(category_id) = cmd.category {
            category = Some(self.repo.get_category_by_id(category_id).await?);
        }
        let transaction = Record::new(cmd.transaction_type, cmd.amount, category, None)?;
        let id = self.repo.create_record(transaction).await?;
        Ok(self.repo.get_record_by_id(id).await?)
    }

    async fn update_record(&self, cmd: UpdateRecordCmd) -> Result<Record> {
        let mut record = self.repo.get_record_by_id(cmd.id).await?;
        let mut category: Option<Category> = None;
        if let Some(category_id) = cmd.category_id {
            category = Some(self.repo.get_category_by_id(category_id).await?);
        }
        record.description = cmd.description;
        record.amount = NonZeroI64::try_from(cmd.amount).map_err(|_| {
            BudgetServiceError::RecordValidationError(RecordError::AmountCannotBeLessOrEqualToZero)
        })?;
        record.category = category;
        record.updated_at = Local::now();

        self.repo.update_record(record).await?;
        Ok(self.repo.get_record_by_id(cmd.id).await?)
    }

    async fn delete_record(&self, id: i64) -> Result<()> {
        self.repo.delete_record(id).await?;
        Ok(())
    }

    async fn list_categories(&self) -> Result<Vec<Category>> {
        Ok(self.repo.list_categories().await?)
    }

    async fn create_category(&self, req: CreateCategoryCmd) -> Result<Category> {
        let category = Category::new(req.name, req.budget, req.parent_id)?;
        let id = self.repo.create_category(category).await?;

        Ok(self.repo.get_category_by_id(id).await?)
    }

    async fn update_category(&self, cmd: UpdateCategoryCmd) -> Result<Category> {
        let mut category = self.repo.get_category_by_id(cmd.id).await?;

        category.name = cmd.name;
        category.budget = cmd.budget;

        self.repo.update_category(category).await?;

        Ok(self.repo.get_category_by_id(cmd.id).await?)
    }

    async fn delete_category(&self, id: i64) -> Result<()> {
        Ok(self.repo.delete_category(id).await?)
    }
}
