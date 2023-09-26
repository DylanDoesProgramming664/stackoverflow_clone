use async_trait::async_trait;
use sqlx::types::Uuid;
use sqlx::PgPool;
use std::sync::Arc;

use crate::models::{DBError, Question, QuestionDetail};

#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: Arc<str>) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        return Self { db };
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let record = sqlx::query!(
            r#"
                INSERT INTO questions (title, description)
                VALUES ( $1, $2 )
                RETURNING *
            "#,
            question.title,
            question.description,
        )
        .fetch_one(&self.db)
        .await
        .map_err(|err| DBError::Other(Box::new(err)))?;

        // Populate the QuestionDetail fields using `record`.
        return Ok(QuestionDetail {
            question_uuid: Arc::from(record.question_uuid.to_string()),
            title: record.title,
            description: record.description,
            created_at: Arc::from(record.created_at.to_string()),
            modified_at: record.modified_at.to_string(),
        });
    }

    async fn delete_question(&self, question_uuid: Arc<str>) -> Result<(), DBError> {
        let uuid = Uuid::parse_str(question_uuid.as_ref()).map_err(|_| {
            DBError::InvalidUUID(format!("Could not parse question_uuid: {}", question_uuid))
        })?;

        sqlx::query!(
            r#"
                DELETE FROM questions WHERE question_uuid = $1
            "#,
            uuid
        )
        .execute(&self.db)
        .await
        .map_err(|err| DBError::Other(Box::new(err)))?;

        return Ok(());
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let records = sqlx::query!("SELECT * FROM questions")
            .fetch_all(&self.db)
            .await
            .map_err(|err| DBError::Other(Box::new(err)))?;

        // Iterate over `records` and map each record to a `QuestionDetail` type
        let questions: Vec<QuestionDetail> = records
            .iter()
            .map(|rec| QuestionDetail {
                question_uuid: Arc::from(rec.question_uuid.to_string()),
                title: rec.title.to_string(),
                description: rec.description.to_string(),
                created_at: Arc::from(rec.created_at.to_string()),
                modified_at: rec.modified_at.to_string(),
            })
            .collect();

        return Ok(questions);
    }
}
