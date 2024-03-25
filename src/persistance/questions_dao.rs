use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{DBError, Question, QuestionDetail};

#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let Question { title, description } = question;

        let record = sqlx::query!(r#"
          INSERT INTO questions (title, description)
          VALUES ($1, $2)
          RETURNING *
        "#, title, description)
        .fetch_all(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        let question_uuid = record[0].question_uuid.to_string();
        let created_at = record[0].created_at.to_string();

        Ok(QuestionDetail {
            question_uuid,
            title,
            description,
            created_at,
        })
    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        let uuid = Uuid::parse_str(&question_uuid).map_err(|e| DBError::InvalidUUID(e.to_string()))?;
        sqlx::query!(r#"DELETE FROM questions WHERE question_uuid = $1"#, uuid).execute(&self.db).await.map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let records = sqlx::query!("Select * from questions").fetch_all(&self.db).await.map_err(|e| DBError::Other(Box::new(e)))?;
        let questions = records.iter().map(|r| QuestionDetail { question_uuid: r.question_uuid.to_string(), title: r.title.clone(), description: r.description.clone(), created_at: r.created_at.to_string() }).collect();

        Ok(questions)
    }
}