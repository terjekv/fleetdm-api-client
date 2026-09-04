use thiserror::Error;

/// Main error type for the FleetDM API client
#[derive(Error, Debug, Clone, PartialEq)]
pub enum FleetError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Http(String),

    /// HTTP request timed out
    #[error("HTTP request timed out: {0}")]
    Timeout(String),

    /// Failed to connect to the Fleet server
    #[error("HTTP connection failed: {0}")]
    Connection(String),

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Fleet accepted the credentials but requires MFA completion outside this flow.
    #[error("Multi-factor authentication required: {0}")]
    MfaRequired(String),

    /// Bad request error (400)
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// API returned an error response with full response details
    #[error("API error: {message} (status: {status})")]
    Api {
        status: u16,
        message: String,
        uuid: Option<String>,
        /// Full error response from the server for detailed error handling
        response: Box<ApiErrorResponse>,
    },

    /// Rate limit exceeded
    #[error("Rate limit exceeded. Try again after: {retry_after:?}")]
    RateLimit {
        retry_after: Option<String>,
        /// Fleet's structured response body, when one was returned.
        response: Option<Box<ApiErrorResponse>>,
    },

    /// Premium license required
    #[error("Fleet Premium license required: {message}")]
    PremiumRequired {
        message: String,
        uuid: Option<String>,
    },

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// A response exceeded the client's in-memory safety limit.
    #[error("Response body exceeded the {limit}-byte in-memory safety limit")]
    ResponseTooLarge { limit: usize },

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// URL parsing error
    #[error("Invalid URL: {0}")]
    UrlParse(String),
}

// Manual From implementations since we removed the #[from] attributes
impl From<reqwest::Error> for FleetError {
    fn from(err: reqwest::Error) -> Self {
        let message = err.to_string();
        if err.is_timeout() {
            FleetError::Timeout(message)
        } else if err.is_connect() {
            FleetError::Connection(message)
        } else {
            FleetError::Http(message)
        }
    }
}

impl From<serde_json::Error> for FleetError {
    fn from(err: serde_json::Error) -> Self {
        FleetError::Serialization(err.to_string())
    }
}

impl From<url::ParseError> for FleetError {
    fn from(err: url::ParseError) -> Self {
        FleetError::UrlParse(err.to_string())
    }
}

/// Response from FleetDM API when an error occurs
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ApiErrorResponse {
    pub message: String,
    #[serde(default)]
    pub errors: Vec<ErrorDetail>,
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ErrorDetail {
    pub name: String,
    pub reason: String,
}

pub type Result<T> = std::result::Result<T, FleetError>;
