use async_trait::async_trait;
use sqlx::{query, types::Uuid, PgPool};
use std::sync::Arc;

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: Arc<str>) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: Arc<str>) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        return Self { db };
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let uuid: Uuid = Uuid::parse_str(answer.question_uuid.as_ref())
            .map_err(|err| DBError::InvalidUUID(err.to_string()))?;

        let record = query!(
            r#"
                INSERT INTO answers ( question_uuid, content )
                VALUES ( $1, $2 )
                RETURNING *
            "#,
            uuid,
            answer.content,
        )
        .fetch_one(&self.db)
        .await
        .map_err(|err: sqlx::Error| match err {
            sqlx::Error::Database(err) => {
                let code = err.code().unwrap();
                if code == postgres_error_codes::FOREIGN_KEY_VIOLATION {
                    DBError::InvalidUUID(format!("Foreign key violation: {}", code))
                } else {
                    DBError::Other(Box::new(err))
                }
            }
            _ => DBError::Other(Box::new(err)),
        })?;

        return Ok(AnswerDetail {
            answer_uuid: Arc::from(record.answer_uuid.to_string()),
            question_uuid: Arc::from(record.question_uuid.to_string()),
            content: record.content,
            created_at: Arc::from(record.created_at.to_string()),
            modified_at: record.modified_at.to_string(),
        });
    }

    async fn delete_answer(&self, answer_uuid: Arc<str>) -> Result<(), DBError> {
        let uuid: Uuid = Uuid::parse_str(answer_uuid.as_ref()).map_err(|uuid| {
            DBError::InvalidUUID(format!("Could not parse answer_uuid: {}", uuid))
        })?;

        query!("DELETE FROM answers WHERE answer_uuid = $1", uuid)
            .execute(&self.db)
            .await
            .map_err(|err| DBError::Other(Box::new(err)))?;

        return Ok(());
    }

    async fn get_answers(&self, question_uuid: Arc<str>) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid: Uuid = Uuid::parse_str(question_uuid.as_ref()).map_err(|uuid| {
            DBError::InvalidUUID(format!("Could not parse question_uuid: {}", uuid))
        })?;

        let records = query!("SELECT * FROM answers WHERE question_uuid = $1", uuid)
            .fetch_all(&self.db)
            .await
            .map_err(|err| DBError::Other(Box::new(err)))?;

        let answers = records
            .iter()
            .map(|rec| AnswerDetail {
                answer_uuid: Arc::from(rec.answer_uuid.to_string()),
                question_uuid: Arc::from(rec.question_uuid.to_string()),
                content: rec.content.to_string(),
                created_at: Arc::from(rec.created_at.to_string()),
                modified_at: rec.modified_at.to_string(),
            })
            .collect();

        return Ok(answers);
    }
}
