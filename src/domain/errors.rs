#[derive(Debug, thiserror::Error)]
pub enum AkariParserError {
    #[error("Daily Akari Score not found")]
    ScoreNotFound,
    #[error("Invalid date format")]
    InvalidDateFormat,
    #[error("Invalid score format")]
    InvalidScoreFormat,
    #[error("Internal error: {0}")]
    InternalError(String),
}