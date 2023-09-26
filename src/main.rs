use anyhow::{Context, Result};
use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::{self, dotenv};
use http::Method;
use pretty_env_logger;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

mod handlers;
mod models;
mod persistance;

use handlers::*;
use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    pretty_env_logger::init();
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .context("Failed to create Postgres connection pool!")?;

    let questions_dao = QuestionsDaoImpl::new(db_pool.clone());
    let answers_dao = AnswersDaoImpl::new(db_pool);

    let questions_dao_state: Arc<(dyn QuestionsDao + Send + Sync)> = Arc::from(questions_dao);
    let answers_dao_state: Arc<(dyn AnswersDao + Send + Sync)> = Arc::from(answers_dao);

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any);

    let questions_app = Router::new()
        .route("/questions", get(read_questions))
        .route("/question", post(create_question).delete(delete_question))
        .with_state(questions_dao_state);

    let answers_app = Router::new()
        .route("/answers", get(read_answers))
        .route("/answer", post(create_answer).delete(delete_answer))
        .with_state(answers_dao_state);

    let app = Router::new()
        .merge(questions_app)
        .merge(answers_app)
        .layer(cors_layer);

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    return Ok(());
}
