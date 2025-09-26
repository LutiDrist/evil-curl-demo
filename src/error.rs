use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyToolError {
    #[error("Wrong wed or HTTP {0}")] // <- this atribut and #[from] is auto trade FROM
    ReqwestError(#[from] reqwest::Error),

    #[error("Wrong wed or HTTP {0}")]
    IoError(#[from] std::io::Error),

    #[error("Wrong URL")]
    InvalidUrl(String),

    #[error("Wrong HTTP method")]
    InvalidMethod(String),

    #[error("Wrong parsing JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Wrong leathers: {0}")]
    InvalidHeader(String),

}