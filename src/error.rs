use {
    crate::{project::ProjectDataError, storage::error::StorageError},
    axum::{response::IntoResponse, Json},
    cerberus::registry::RegistryError,
    hyper::StatusCode,
    tracing::log::{error, warn},
};

pub type RpcResult<T> = Result<T, RpcError>;

#[derive(Debug, thiserror::Error)]
pub enum RpcError {
    #[error(transparent)]
    EnvyError(#[from] envy::Error),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Project data error: {0}")]
    ProjectDataError(#[from] ProjectDataError),

    #[error("Registry error")]
    RegistryError(#[from] RegistryError),

    #[error("Storage error")]
    StorageError(#[from] StorageError),

    #[error("Chain not found despite previous validation")]
    ChainNotFound,

    #[error("Transport error: {0}")]
    TransportError(#[from] hyper::Error),

    #[error("Request::builder() failed: {0}")]
    RequestBuilderError(#[from] hyper::http::Error),

    #[error("Specified chain is not supported by any of the providers")]
    UnsupportedChain(String),

    #[error("Provider is throttling the requests")]
    Throttled,

    #[error("Failed to reach the provider")]
    ProviderError,

    #[error(transparent)]
    Cerberus(#[from] cerberus::project::AccessError),

    #[error("{0:?}")]
    Other(#[from] anyhow::Error),

    #[error("Invalid scheme used. Try http(s):// or ws(s)://")]
    InvalidScheme,

    #[error(transparent)]
    AxumTungstenite(#[from] axum_tungstenite::Error),
}

impl IntoResponse for RpcError {
    fn into_response(self) -> axum::response::Response {
        error!("{:?}", self);
        match self {
            Self::AxumTungstenite(err) => (StatusCode::GONE, err.to_string()).into_response(),
            Self::UnsupportedChain(chain_id) => (
                StatusCode::BAD_REQUEST,
                Json(new_error_response(
                    "chainId".to_string(),
                    format!("We don't support the chainId you provided: {chain_id}"),
                )),
            )
                .into_response(),
            Self::ProviderError => (
                StatusCode::BAD_REQUEST,
                Json(new_error_response(
                    "unreachable".to_string(),
                    "We failed to reach the provider for your request".to_string(),
                )),
            )
                .into_response(),
            Self::InvalidScheme => (
                StatusCode::BAD_REQUEST,
                "Invalid scheme used. Try http(s):// or ws(s)://".to_string(),
            )
                .into_response(),
            Self::RegistryError(e) => {
                warn!("Registry error: {:?}", e);
                (
                    StatusCode::UNAUTHORIZED,
                    "We failed to authenticate your request".to_string(),
                )
                    .into_response()
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
                .into_response(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct ErrorReason {
    pub field: String,
    pub description: String,
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub reasons: Vec<ErrorReason>,
}

pub fn new_error_response(field: String, description: String) -> ErrorResponse {
    ErrorResponse {
        status: "FAILED".to_string(),
        reasons: vec![ErrorReason { field, description }],
    }
}
