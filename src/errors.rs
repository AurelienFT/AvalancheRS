use std::fmt;

#[derive(Debug, Clone)]
pub struct AvalancheError;


impl fmt::Display for AvalancheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}