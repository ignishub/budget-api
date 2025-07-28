use async_trait::async_trait;

use crate::{
    domain::{
        Result,
        models::{Account, Category, Record},
    },
    service::{
        accounts::BudgetAccountsService,
        categories::BudgetCategoriesService,
        records::{BudgetRecordService, ListRecordsCmd},
    },
};

#[async_trait]
pub trait AccountRepository: Clone + Sync + 'static {
    async fn list_accounts(&self) -> Result<Vec<Account>>;
    async fn create_account(&self, acc: Account) -> Result<i64>;
    async fn get_account_by_id(&self, id: i64) -> Result<Account>;
    async fn update_account(&self, acc: Account) -> Result<()>;
    async fn delete_account(&self, id: i64) -> Result<()>;
}

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

#[async_trait]
pub trait UserRepository: Clone + Send + Sync + 'static {}

pub trait BudgetRepository: RecordRepository + CategoryRepository + AccountRepository {}

pub trait BudgetService:
    BudgetAccountsService + BudgetRecordService + BudgetCategoriesService
{
}

impl<T: BudgetRepository> BudgetService for BudgetServiceImpl<T> {}

#[derive(Clone)]
pub struct BudgetServiceImpl<T: BudgetRepository> {
    pub repo: T,
}

impl<T: BudgetRepository> BudgetServiceImpl<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}
