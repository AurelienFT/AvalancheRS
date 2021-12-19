use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum AvalancheError {
    #[error("JSON RPC call {call:?} failed (code: {code:?}, message: {message:?})")]
    ErrorJsonRpcCall {
        call: String,
        code: String,
        message: String,
    },
    #[error("API {api:?} is not initialized.")]
    ApiNotInitialized {
        api: String
    },
    #[error("Protocol not supported.")]
    BadProtocol,
    #[error("Unknown Error")]
    Unknown,
}

//TODO: Improve
impl From<hyper::Error> for AvalancheError {
    fn from(_error: hyper::Error) -> Self {
        AvalancheError::Unknown
    }
}

impl From<tokio::io::Error> for AvalancheError {
    fn from(_error: tokio::io::Error) -> Self {
        AvalancheError::Unknown
    }
}