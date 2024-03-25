use std::{borrow::Cow, error::Error};

use async_trait::async_trait;
use sqlx::{postgres::PgDatabaseError, PgPool};
use uuid::Uuid;

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let Answer {
            question_uuid,
            content,
        } = answer;
        let uuid =
            Uuid::parse_str(&question_uuid).map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        let record = sqlx::query!(
            r#"INSERT INTO answers (question_uuid, content) VALUES ( $1, $2 ) returning *"#,
            uuid,
            content
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| {
            if let Some(err) = e.as_database_error() {
                if err.code() == Some(Cow::Borrowed(postgres_error_codes::FOREIGN_KEY_VIOLATION)) {
                    return DBError::InvalidUUID(err.to_string());
                }
            }
            DBError::Other(Box::new(e))
        })?;

        let answer_uuid = record[0].answer_uuid.to_string();
        let question_uuid = record[0].question_uuid.to_string();
        let content = record[0].content.clone();
        let created_at = record[0].created_at.to_string();

        Ok(AnswerDetail {
            answer_uuid,
            question_uuid,
            content,
            created_at,
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid =
            Uuid::parse_str(&answer_uuid).map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        sqlx::query!("DELETE FROM answers WHERE answer_uuid = $1", uuid)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;
        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid =
            Uuid::parse_str(&question_uuid).map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        let records = sqlx::query!("SELECT * FROM answers WHERE question_uuid = $1", uuid)
            .fetch_all(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        let answers = records
            .iter()
            .map(|r| AnswerDetail {
                answer_uuid: r.answer_uuid.to_string(),
                question_uuid: r.question_uuid.to_string(),
                content: r.content.clone(),
                created_at: r.created_at.to_string(),
            })
            .collect();

        Ok(answers)
    }
}
