use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranslationError {
    #[error("model download failed: {0}")]
    DownloadError(String),
    #[error("model initialization failed: {0}")]
    InitError(String),
    #[error("translation failed: {0}")]
    TranslationFailed(String),
    #[error("tokenization failed: {0}")]
    TokenizationError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
