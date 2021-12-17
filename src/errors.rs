use std::fmt;

#[derive(Debug, Clone)]
pub struct AvalancheError;

impl fmt::Display for AvalancheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

//TODO: Improve
impl From<hyper::Error> for AvalancheError {
    fn from(_error: hyper::Error) -> Self {
        AvalancheError
    }
}

impl From<tokio::io::Error> for AvalancheError {
    fn from(_error: tokio::io::Error) -> Self {
        AvalancheError
    }
}