use std::collections::HashMap;

use crate::models::*;
use axum::{ http::StatusCode, response::IntoResponse, Json };
use chrono::Utc;
use uuid::Uuid;

// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
  let Question { title, description } = question;
  let question_uuid = Uuid::new_v4().to_string();
  let created_at = Utc::now().to_string();
  let question_detail = QuestionDetail {
    question_uuid,
    title,
    description,
    created_at,
  };
  (StatusCode::OK, Json(question_detail))
}

pub async fn read_questions() -> impl IntoResponse {
    (StatusCode::OK, Json::<Vec<QuestionDetail>>(vec![
      QuestionDetail {
          question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_owned(),
          title: "Newly Created Question".to_owned(),
          description: "My Description".to_owned(),
          created_at: "2022-12-31 18:44:08.287442".to_owned()
      }
  ]))
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) {
  let QuestionId { question_uuid } = question_uuid;
  let mut questions: HashMap<String, QuestionDetail> = HashMap::new();
  questions.insert(question_uuid.clone(), QuestionDetail {
    created_at: Utc::now().to_string(), 
    description: "description".to_owned(), 
    title: "title".to_owned(), 
    question_uuid: question_uuid.clone()
  });
  questions.remove(&question_uuid);

}

// ---- CRUD for Answers ----

pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    let Answer { content, question_uuid } = answer;
    let answer_uuid = Uuid::new_v4().to_string();
    let created_at = Utc::now().to_string();
    let answer_detail = AnswerDetail {
      answer_uuid,
      content,
      question_uuid,
      created_at,
    };
    (StatusCode::OK, Json(answer_detail))
}

pub async fn read_answers() -> impl IntoResponse {
  (StatusCode::OK, Json::<Vec<AnswerDetail>>(vec![
    AnswerDetail {
        answer_uuid: "a1a14a9c-ab9e-481b-8120-67f675531ed2".to_owned(),
        created_at: "2022-12-31 18:44:08.287442".to_owned(),
        content: "Test question".to_owned(),
        question_uuid: "d347261c-3f0e-42d2-8706-5ef9f1b96725".to_owned()
    }
]))
}

pub async fn delete_answer(Json(answer_uuid): Json<AnswerId>) {
  let AnswerId { answer_uuid } = answer_uuid;
  let mut answers: HashMap<String, AnswerDetail> = HashMap::new();
  answers.insert(answer_uuid.clone(), AnswerDetail {
    created_at: Utc::now().to_string(), 
    content: "content".to_owned(), 
    question_uuid: "".to_owned(),
    answer_uuid: answer_uuid.clone()
  });
  answers.remove(&answer_uuid);
}