use async_trait::async_trait;

use crate::{
    domain::{Result, models::Account},
    repository::{
        SqliteBudgetRepo,
        dto::{AccountDTO, ReturnedId},
    },
    service::budget::AccountRepository,
};

#[async_trait]
impl AccountRepository for SqliteBudgetRepo {
    async fn list_accounts(&self) -> Result<Vec<Account>> {
        let mut conn = self.pool.acquire().await?;

        let result = sqlx::query_as::<_, AccountDTO>(
            r#"
            SELECT account_id,name,current_balance, name, account_type
            FROM account
            "#,
        )
        .fetch_all(&mut *conn)
        .await?;

        let result: Vec<Account> = result.into_iter().map(Account::from).collect();

        Ok(result)
    }

    async fn create_account(&self, acc: Account) -> Result<i64> {
        let mut conn = self.pool.acquire().await?;

        let result = sqlx::query_as::<_, ReturnedId>(
            r#"
            INSERT INTO account
            (name,current_balance,account_type)
            VALUES(?,?,?)
            RETURNING account_id as id;
            "#,
        )
        .bind(acc.name)
        .bind(acc.balance)
        .bind(acc.account_type.to_string())
        .fetch_one(&mut *conn)
        .await?;

        Ok(result.id)
    }

    async fn get_account_by_id(&self, id: i64) -> Result<Account> {
        let mut conn = self.pool.acquire().await?;

        let result = sqlx::query_as::<_, AccountDTO>(
            r#"
            SELECT account_id,name,current_balance, name, account_type
            FROM account
            WHERE account_id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&mut *conn)
        .await?;

        Ok(result.into())
    }

    async fn update_account(&self, acc: Account) -> Result<()> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query(
            r#"
            UPDATE account
                SET name = ?,
                    account_type = ?
            WHERE account_id = ?
            "#,
        )
        .bind(acc.name)
        .bind(acc.account_type.to_string())
        .bind(acc.id)
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    async fn delete_account(&self, id: i64) -> Result<()> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query(
            r#"
            DELETE 
            FROM account
            WHERE account_id = ?
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
    use sqlx::types::chrono::NaiveDate;

    use crate::repository::test::test_db;

    use super::*;

    #[tokio::test]
    async fn test_list_accounts() {
        let fixture = include_str!("./fixtures/fixture.sql");
        let repo = test_db(Some(fixture)).await;

        let mut account = Account::new("test account".into(), 0, "Cash".into()).unwrap();
        account.id = 1;
        let list = vec![account];
        let result = repo.list_accounts().await;
        assert!(result.is_ok(), "{}", result.err().unwrap());
        assert_eq!(result.unwrap(), list)
    }

    #[tokio::test]
    async fn test_create_category() {
        let fixture = include_str!("./fixtures/fixture.sql");
        let repo = test_db(Some(fixture)).await;

        let account = Account::new("test account".into(), 0, "Cash".into()).unwrap();
        let result = repo.create_account(account).await;
        assert!(result.is_ok(), "{}", result.err().unwrap());
        assert!(result.unwrap() > 0);
    }

    #[tokio::test]
    async fn test_get_category_by_id() {
        let fixture = include_str!("./fixtures/fixture.sql");
        let repo = test_db(Some(fixture)).await;

        let mut account = Account::new("test account".into(), 0, "Cash".into()).unwrap();
        account.id = 1;
        let found_account = repo.get_account_by_id(1).await.expect("must find category");

        assert_eq!(account, found_account);
    }

    #[tokio::test]
    async fn test_update_category() {
        let fixture = include_str!("./fixtures/fixture.sql");
        let repo = test_db(Some(fixture)).await;

        let mut account = Account::new("test account".into(), 0, "Cash".into()).unwrap();
        account.id = 1;
        let result = repo.update_account(account.clone()).await;
        assert!(result.is_ok(), "{}", result.err().unwrap());

        let updated_account = repo.get_account_by_id(1).await.expect("must find category");

        assert_eq!(account, updated_account);
    }

    #[tokio::test]
    async fn test_delete_category() {
        let fixture = include_str!("./fixtures/fixture.sql");
        let repo = test_db(Some(fixture)).await;

        let result = repo.delete_account(1).await;
        assert!(result.is_ok(), "{}", result.err().unwrap());

        let result = repo.get_account_by_id(1).await;
        assert!(result.is_err(), "{}", result.err().unwrap());
    }
}
