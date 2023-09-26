use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionDetail {
    pub question_uuid: Arc<str>,
    pub title: String,
    pub description: String,
    pub created_at: Arc<str>,
    pub modified_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct QuestionId {
    pub question_uuid: Arc<str>,
}

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub question_uuid: Arc<str>,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerDetail {
    pub answer_uuid: Arc<str>,
    pub question_uuid: Arc<str>,
    pub content: String,
    pub created_at: Arc<str>,
    pub modified_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct AnswerID {
    pub answer_uuid: Arc<str>,
}

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Invalid UUID provided: {0}")]
    InvalidUUID(String),
    #[error("Database error occurred")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

// source: https://www.postgresql.org/docs/current/errcodes-appendix.html
pub mod postgres_error_codes {
    pub const FOREIGN_KEY_VIOLATION: &str = "23503";
}
