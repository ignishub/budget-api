use async_trait::async_trait;

use crate::domain::Result;
use crate::domain::models::Category;
use crate::repository::SqliteBudgetRepo;
use crate::repository::dto::{CategoryDTO, ReturnedId};
use crate::service::budget::CategoryRepository;

#[async_trait]
impl CategoryRepository for SqliteBudgetRepo {
    async fn list_categories(&self) -> Result<Vec<Category>> {
        let mut conn = self.pool.acquire().await?;

        let result = sqlx::query_as::<_, CategoryDTO>(
            r#"
            SELECT category_id,name,budget, parent_id
            FROM category
            "#,
        )
        .fetch_all(&mut *conn)
        .await?;

        let result: Vec<Category> = result.into_iter().map(Category::from).collect();

        Ok(result)
    }

    async fn create_category(&self, category: Category) -> Result<i64> {
        let mut conn = self.pool.acquire().await?;

        let result = sqlx::query_as::<_, ReturnedId>(
            r#"
            INSERT INTO category
            (name,budget,parent_id)
            VALUES(?,?,?)
            RETURNING category_id as id;
            "#,
        )
        .bind(category.name)
        .bind(category.budget)
        .bind(category.parent_id)
        .fetch_one(&mut *conn)
        .await?;

        Ok(result.id)
    }

    async fn get_category_by_id(&self, id: i64) -> Result<Category> {
        let mut conn = self.pool.acquire().await?;

        let result = sqlx::query_as::<_, CategoryDTO>(
            r#"
            SELECT category_id,name,budget, parent_id
            FROM category
            where category_id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&mut *conn)
        .await?;

        Ok(result.into())
    }

    async fn update_category(&self, category: Category) -> Result<()> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query(
            r#"
            UPDATE categoty
                SET name = ?
                SET budget = ?
            WHERE category_id = ?
            "#,
        )
        .bind(category.name)
        .bind(category.budget)
        .bind(category.id)
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    async fn delete_category(&self, id: i64) -> Result<()> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query(
            r#"
            DELETE 
            FROM category
            WHERE category_id = ?
            "#,
        )
        .bind(id)
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::repository::test::test_db;

    use super::*;

    #[tokio::test]
    async fn test_create_category() {
        let pool = test_db().await;
        let repo = SqliteBudgetRepo::new(pool);

        let category = Category::new("test".into(), Some(1000), None).unwrap();
        let result = repo.create_category(category).await;
        assert!(result.is_ok(), "{}", result.err().unwrap());
        assert!(result.unwrap() > 0);
    }
}
