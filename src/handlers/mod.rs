use std::sync::Arc;

use crate::{
    models::*,
    persistance::{answers_dao::AnswersDao, questions_dao::QuestionsDao},
};
use axum::{extract::State, Json};

pub mod handlers_inner;

pub enum APIError {}

pub async fn create_question(
    State(questions_dao): State<Arc<(dyn QuestionsDao + Send + Sync)>>,
    Json(question): Json<Question>,
) -> Json<QuestionDetail> {
    return Json::from(
        handlers_inner::create_question(question, questions_dao)
            .await
            .unwrap(),
    );
}

pub async fn read_questions(
    State(questions_dao): State<Arc<(dyn QuestionsDao + Send + Sync)>>,
) -> Json<Vec<QuestionDetail>> {
    return Json::from(handlers_inner::read_questions(questions_dao).await.unwrap());
}

pub async fn delete_question(
    State(questions_dao): State<Arc<(dyn QuestionsDao + Send + Sync)>>,
    Json(question_uuid): Json<QuestionId>,
) {
    let db = handlers_inner::delete_question(question_uuid, questions_dao)
        .await
        .unwrap();
    return db;
}

pub async fn create_answer(
    State(answers_dao): State<Arc<(dyn AnswersDao + Send + Sync)>>,
    Json(answer): Json<Answer>,
) -> Json<AnswerDetail> {
    return Json::from(
        handlers_inner::create_answer(answer, answers_dao)
            .await
            .unwrap(),
    );
}

pub async fn read_answers(
    State(answers_dao): State<Arc<(dyn AnswersDao + Send + Sync)>>,
    Json(question_uuid): Json<QuestionId>,
) -> Json<Vec<AnswerDetail>> {
    return Json::from(
        handlers_inner::read_answers(question_uuid, answers_dao)
            .await
            .unwrap(),
    );
}

pub async fn delete_answer(
    State(answers_dao): State<Arc<(dyn AnswersDao + Send + Sync)>>,
    Json(answer_uuid): Json<AnswerID>,
) {
    handlers_inner::delete_answer(answer_uuid, answers_dao)
        .await
        .unwrap();
}
