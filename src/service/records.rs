use async_trait::async_trait;
use sqlx::types::chrono::Local;

use crate::{
    domain::{
        Result,
        models::{Category, Record},
    },
    service::budget::{BudgetRepository, BudgetServiceImpl},
};

#[derive(Debug)]
pub struct CreateRecordCmd {
    pub account_id: i64,
    pub transaction_type: String,
    pub amount: i64,
    pub category: Option<i64>,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct ListRecordsCmd {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub category_id: Option<i64>,
}

pub struct UpdateRecordCmd {
    pub id: i64,
    pub amount: i64,
    pub description: Option<String>,
    pub category_id: Option<i64>,
}
#[async_trait]
pub trait BudgetRecordService: Send + Sync + 'static {
    async fn list_records(&self, cmd: ListRecordsCmd) -> Result<Vec<Record>>;
    async fn create_record(&self, cmd: CreateRecordCmd) -> Result<Record>;
    async fn update_record(&self, cmd: UpdateRecordCmd) -> Result<Record>;
    async fn delete_record(&self, id: i64) -> Result<()>;
}

#[async_trait]
impl<T: BudgetRepository> BudgetRecordService for BudgetServiceImpl<T> {
    async fn list_records(&self, cmd: ListRecordsCmd) -> Result<Vec<Record>> {
        Ok(self.repo.list_records(cmd).await?)
    }

    async fn create_record(&self, cmd: CreateRecordCmd) -> Result<Record> {
        let mut category: Option<Category> = None;
        if let Some(category_id) = cmd.category {
            category = Some(self.repo.get_category_by_id(category_id).await?);
        }
        let transaction = Record::new(
            cmd.account_id,
            cmd.transaction_type,
            cmd.amount,
            category,
            cmd.description,
        )?;
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
        record.set_amount(cmd.amount)?;
        record.category = category;
        record.updated_at = Local::now();

        self.repo.update_record(record).await?;
        Ok(self.repo.get_record_by_id(cmd.id).await?)
    }

    async fn delete_record(&self, id: i64) -> Result<()> {
        self.repo.delete_record(id).await?;
        Ok(())
    }
}
