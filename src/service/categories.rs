use async_trait::async_trait;

use crate::{
    domain::{Result, models::Category},
    service::budget::{BudgetRepository, BudgetServiceImpl},
};

pub struct CreateCategoryCmd {
    pub name: String,
    pub budget: Option<i64>,
    pub parent_id: Option<i64>,
}

pub struct UpdateCategoryCmd {
    pub id: i64,
    pub name: String,
    pub budget: Option<i64>,
}

#[async_trait]
pub trait BudgetCategoriesService: Send + Sync + 'static {
    async fn list_categories(&self) -> Result<Vec<Category>>;
    async fn create_category(&self, cmd: CreateCategoryCmd) -> Result<Category>;
    async fn update_category(&self, cmd: UpdateCategoryCmd) -> Result<Category>;
    async fn delete_category(&self, id: i64) -> Result<()>;
}

#[async_trait]
impl<T: BudgetRepository> BudgetCategoriesService for BudgetServiceImpl<T> {
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
