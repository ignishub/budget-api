use async_trait::async_trait;

use crate::{
    domain::{Result, models::Account},
    service::budget::{BudgetRepository, BudgetServiceImpl},
};

#[async_trait]
pub trait BudgetAccountsService: Send + Sync + 'static {
    async fn list_accounts(&self) -> Result<Vec<Account>>;
    async fn create_account(&self, cmd: CreateAccountCmd) -> Result<Account>;
    async fn update_account(&self, cmd: UpdateAccountCmd) -> Result<Account>;
    async fn delete_account(&self, id: i64) -> Result<()>;
}

pub struct CreateAccountCmd {
    pub name: String,
    pub account_type: String,
    pub initial_balance: i64,
}

pub struct UpdateAccountCmd {
    pub id: i64,
    pub name: String,
}

#[async_trait]
impl<T: BudgetRepository> BudgetAccountsService for BudgetServiceImpl<T> {
    async fn list_accounts(&self) -> Result<Vec<Account>> {
        Ok(self.repo.list_accounts().await?)
    }

    async fn create_account(&self, cmd: CreateAccountCmd) -> Result<Account> {
        let acc = Account::new(cmd.name, cmd.initial_balance, cmd.account_type)?;
        let acc_id = self.repo.create_account(acc).await?;
        Ok(self.repo.get_account_by_id(acc_id).await?)
    }

    async fn update_account(&self, cmd: UpdateAccountCmd) -> Result<Account> {
        let mut acc = self.repo.get_account_by_id(cmd.id).await?;
        acc.name = cmd.name;
        self.repo.update_account(acc).await?;
        Ok(self.repo.get_account_by_id(cmd.id).await?)
    }

    async fn delete_account(&self, id: i64) -> Result<()> {
        self.repo.delete_account(id).await?;
        Ok(())
    }
}
