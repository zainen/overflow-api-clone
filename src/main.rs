// TODO: import log, pretty_env_logger, dotenv, and PgPoolOptions
extern crate pretty_env_logger;

#[macro_use] extern crate log;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use axum::{
    routing::{delete, get, post},
    Router,
};

mod handlers;
mod models;
mod persistance;

use handlers::*;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    info!("Connecting to database: {}", database_url);

    // Create a new PgPoolOptions instance with a maximum of 5 connections.
    let pool = PgPoolOptions::new()
     .max_connections(5)
     .connect(&database_url)
     .await
     .expect("failed to connect to database");

    // let recs = sqlx::query!(r#"SELECT * FROM questions"#)
    // .fetch_all(&pool)
    // .await
    // .unwrap();

    // info!("********* Question Records *********");


    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}