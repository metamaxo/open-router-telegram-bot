#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Invalid Command: {0}")]
    InvalidCommand(String),
    // NOTE: Je kan hier je eigen error types toevoegen
    // Er zijn twee soorten errors:
    // 1. Errors die je zelf definieert, zoals hierboven Generic
    // 2. Errors die je van andere libraries overneemt, zoals Http en Json
}
