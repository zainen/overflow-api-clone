use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{models::*, persistance::answers_dao, AppState};

mod handlers_inner;

impl IntoResponse for handlers_inner::HandlerError {
  fn into_response(self) -> axum::response::Response {
      match self {
          handlers_inner::HandlerError::BadRequest(msg) => {
              (StatusCode::BAD_REQUEST, msg).into_response()
          }
          handlers_inner::HandlerError::InternalError(msg) => {
              (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
          }
      }
  }
}

// ---- CRUD for Questions ----

pub async fn create_question(
    // Example of how to add state to a route. Note that we are using ".." to ignore the other fields in AppState.
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>,
) -> impl IntoResponse {
    let questions_dao = questions_dao.as_ref();
    let question = handlers_inner::create_question(question, questions_dao).await;
    match question {
      Ok(question) => Json(question).into_response(),
      Err(error) => error.into_response(),
    }
}

pub async fn read_questions(// TODO: add questions_dao from app state as an argument
  State(AppState { questions_dao, ..}): State<AppState> 
) -> impl IntoResponse {
  let questions = handlers_inner::read_questions(questions_dao.as_ref()).await;
  match questions {
    Ok(questions) => Json(questions).into_response(),
    Err(error) => error.into_response(),
  }
}

pub async fn delete_question(
    // TODO: add questions_dao from app state as an argument
    State(AppState { questions_dao, ..}): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> impl IntoResponse {
    match handlers_inner::delete_question(question_uuid, questions_dao.as_ref()).await {
      Ok(_) => StatusCode::OK,
      Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    // Example of how to add state to a route
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> impl IntoResponse {
    let answers_dao = answers_dao.as_ref();
    let answer = handlers_inner::create_answer(answer, answers_dao).await;
    match answer {
      Ok(answer) => Json(answer).into_response(),
      Err(error) => error.into_response(),
    }
}

pub async fn read_answers(
    // TODO: add answers_dao from app state as an argument
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> impl IntoResponse {
  let answers = handlers_inner::read_answers(question_uuid, answers_dao.as_ref()).await;
  match answers {
    Ok(answers) => Json(answers).into_response(),
    Err(error) => error.into_response(),
  }
}

pub async fn delete_answer(
    // TODO: add answers_dao from app state as an argument
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer_uuid): Json<AnswerId>,
) -> impl IntoResponse {
  match handlers_inner::delete_answer(answer_uuid, answers_dao.as_ref()).await {
    Ok(_) => StatusCode::OK,
    Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
  }
}