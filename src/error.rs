use std::fmt;

#[derive(Debug)]
pub enum AppError {
    #[allow(dead_code)]
    TransmissionError(Box<dyn std::error::Error + Send + Sync>),
    HttpError(reqwest::Error),
    EnvironmentError(String),
    #[allow(dead_code)]
    ParseError(String),
    #[allow(dead_code)]
    NetworkError(String),
    #[allow(dead_code)]
    ValidationError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::TransmissionError(e) => write!(f, "Transmission error: {}", e),
            AppError::HttpError(e) => write!(f, "HTTP error: {}", e),
            AppError::EnvironmentError(msg) => write!(f, "Environment error: {}", msg),
            AppError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AppError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// Remove the transmission-rpc specific From implementation since 
// we're using a more generic error type

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        AppError::HttpError(error)
    }
}

impl From<std::env::VarError> for AppError {
    fn from(error: std::env::VarError) -> Self {
        AppError::EnvironmentError(error.to_string())
    }
}

#[allow(dead_code)]
pub type AppResult<T> = Result<T, AppError>;