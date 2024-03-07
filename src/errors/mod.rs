use axum::response::IntoResponse;

/// ReciteError enumerates all possible errors returned by this library.
#[derive(thiserror::Error, Debug)]
pub enum RichTextServerError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
    #[error(transparent)]
    AxumError(#[from] axum::Error),
    #[error(transparent)]
    EnvVariableError(#[from] std::env::VarError),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
}

pub type Result<T> = color_eyre::eyre::Result<T, RichTextServerError>;
