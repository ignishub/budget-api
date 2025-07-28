use async_trait::async_trait;
use sqlx::QueryBuilder;

use crate::{
    domain::{Result, models::Record},
    repository::{
        SqliteBudgetRepo,
        dto::{FullRecordDTO, ReturnedId},
    },
    service::budget::{ListRecordsCmd, RecordRepository},
};

#[async_trait]
impl RecordRepository for SqliteBudgetRepo {
    async fn create_record(&self, record: Record) -> Result<i64> {
        let mut conn = self.pool.acquire().await?;

        let result = sqlx::query_as::<_, ReturnedId>(
            r#"
            INSERT INTO record 
                (amount,record_type,category_id,created_at,updated_at)
            VALUES
                (?,?,?,?,?)
            RETURNING record_id as id;
            "#,
        )
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
                SET description = ?
                SET category_id = ?
                SET record_type = ?
                SET updated_at = ?
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
    use super::*;
    use crate::repository::test::test_db;

    #[tokio::test]
    async fn test_create_category() {
        let pool = test_db().await;
        let repo = SqliteBudgetRepo::new(pool);

        let record = Record::new("Income".into(), 1000, None, Some("text".into())).unwrap();

        let result = repo.create_record(record).await;
        assert!(result.is_ok(), "{}", result.err().unwrap());
        assert!(result.unwrap() > 0);
    }
}
