use async_trait::async_trait;
use sqlx::QueryBuilder;

use crate::{
    domain::{Result, models::Record},
    repository::{
        SqliteBudgetRepo,
        dto::{FullRecordDTO, ReturnedId},
    },
    service::{budget::RecordRepository, records::ListRecordsCmd},
};

#[async_trait]
impl RecordRepository for SqliteBudgetRepo {
    async fn create_record(&self, record: Record) -> Result<i64> {
        let mut conn = self.pool.acquire().await?;

        let result = sqlx::query_as::<_, ReturnedId>(
            r#"
            INSERT INTO record 
                (account_id,amount,record_type,category_id,created_at,updated_at)
            VALUES
                (?,?,?,?,?,?)
            RETURNING record_id as id;
            "#,
        )
        .bind(record.account_id)
        .bind(record.amount)
        .bind(Into::<i64>::into(record.record_type))
        .bind(record.category.map(|c| c.id))
        .bind(record.created_at)
        .bind(record.updated_at)
        .fetch_one(&mut *conn)
        .await?;

        Ok(result.id)
    }

    async fn get_record_by_id(&self, id: i64) -> Result<Record> {
        let mut conn = self.pool.acquire().await?;

        let record = sqlx::query_as::<_, FullRecordDTO>(
            r#"
            SELECT 
                record.record_id,
                record.account_id,
                record.amount,
                record.description,
                record_type.name as 'record_type',
                record.created_at,
                record.updated_at,
                category.category_id,
                category.name,
                category.parent_id
            FROM record
            JOIN record_type ON record.record_type = record_type.record_type_id
            JOIN category ON record.category_id = category.category_id
            WHERE record.record_id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&mut *conn)
        .await?;

        Ok(record.into())
    }

    async fn list_records(&self, req: ListRecordsCmd) -> Result<Vec<Record>> {
        let mut conn = self.pool.acquire().await?;

        let mut query = QueryBuilder::new(
            r#"
            SELECT 
                record.record_id,
                record.account_id,
                record.amount,
                record.description,
                record_type.name as 'record_type',
                record.created_at,
                record.updated_at,
                category.category_id,
                category.name,
                category.parent_id
            FROM record
            JOIN record_type ON record.record_type = record_type.record_type_id
            LEFT JOIN category ON record.category_id = category.category_id
            "#,
        );

        if let Some(category_id) = req.category_id {
            query.push("WHERE record.category_id = ");
            query.push_bind(category_id);
        }

        if let Some(limit) = req.limit {
            query.push("LIMIT ");
            query.push_bind(limit as i64);
        }

        if let Some(offset) = req.offset {
            query.push("OFFSET ");
            query.push_bind(offset as i64);
        }

        let result = query
            .build_query_as::<FullRecordDTO>()
            .fetch_all(&mut *conn)
            .await?;

        Ok(result.into_iter().map(Record::from).collect())
    }

    async fn update_record(&self, record: Record) -> Result<()> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query(
            r#"
            UPDATE record
                SET amount = ?
                    description = ?
                    category_id = ?
                    record_type = ?
                    updated_at = ?
            WHERE category_id = ?
            "#,
        )
        .bind(record.amount)
        .bind(record.description)
        .bind(record.category.map(|c| c.id))
        .bind(Into::<i64>::into(record.record_type))
        .bind(record.updated_at)
        .bind(record.id)
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    async fn delete_record(&self, id: i64) -> Result<()> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query(
            r#"
            DELETE 
            FROM record
            WHERE record_id = ?
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
    use sqlx::types::chrono::DateTime;

    use crate::repository::test::test_db;

    use super::*;

    #[tokio::test]
    async fn test_list_records() {
        let fixture = include_str!("./fixtures/fixture.sql");
        let repo = test_db(Some(fixture)).await;

        let mut record = Record::new(1, "Outcome".into(), 1000, None, Some("test record".into()))
            .expect("error");

        let d =
            DateTime::parse_from_str("2025-08-24 00:00:00 +00:00", "%Y-%m-%d %H:%M:%S %z").unwrap();
        record.id = 1;
        record.created_at = d.into();
        record.updated_at = d.into();
        let list = vec![record];
        let result = repo
            .list_records(ListRecordsCmd {
                limit: None,
                offset: None,
                category_id: None,
            })
            .await;
        assert!(result.is_ok(), "{}", result.err().unwrap());
        assert_eq!(result.unwrap(), list)
    }

    // #[tokio::test]
    // async fn test_create_record() {
    //     let fixture = include_str!("./fixtures/fixture.sql");
    //     let repo = test_db(Some(fixture)).await;
    //
    //     let record = Record::new(1, "Income".into(), 1000, None, Some("text".into())).unwrap();
    //
    //     let result = repo.create_record(record).await;
    //     assert!(result.is_ok(), "{}", result.err().unwrap());
    //     assert!(result.unwrap() > 0);
    // }
    // #[tokio::test]
    // async fn test_create_category() {
    //     let fixture = include_str!("./fixtures/fixture.sql");
    //     let repo = test_db(Some(fixture)).await;
    //
    //     let account = Account::new("test account".into(), 0, "Cash".into()).unwrap();
    //     let result = repo.create_account(account).await;
    //     assert!(result.is_ok(), "{}", result.err().unwrap());
    //     assert!(result.unwrap() > 0);
    // }
    //
    // #[tokio::test]
    // async fn test_get_category_by_id() {
    //     let fixture = include_str!("./fixtures/fixture.sql");
    //     let repo = test_db(Some(fixture)).await;
    //
    //     let mut account = Account::new("test account".into(), 0, "Cash".into()).unwrap();
    //     account.id = 1;
    //     let found_account = repo.get_account_by_id(1).await.expect("must find category");
    //
    //     assert_eq!(account, found_account);
    // }
    //
    // #[tokio::test]
    // async fn test_update_category() {
    //     let fixture = include_str!("./fixtures/fixture.sql");
    //     let repo = test_db(Some(fixture)).await;
    //
    //     let mut account = Account::new("test account".into(), 0, "Cash".into()).unwrap();
    //     account.id = 1;
    //     let result = repo.update_account(account.clone()).await;
    //     assert!(result.is_ok(), "{}", result.err().unwrap());
    //
    //     let updated_account = repo.get_account_by_id(1).await.expect("must find category");
    //
    //     assert_eq!(account, updated_account);
    // }
    //
    // #[tokio::test]
    // async fn test_delete_category() {
    //     let fixture = include_str!("./fixtures/fixture.sql");
    //     let repo = test_db(Some(fixture)).await;
    //
    //     let result = repo.delete_account(1).await;
    //     assert!(result.is_ok(), "{}", result.err().unwrap());
    //
    //     let result = repo.get_account_by_id(1).await;
    //     assert!(result.is_err(), "{}", result.err().unwrap());
    // }
}
