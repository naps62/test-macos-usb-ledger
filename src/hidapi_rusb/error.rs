use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum HidError {
    InitializationError,
}

impl Display for HidError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            HidError::InitializationError => {
                write!(f, "Failed to initialize hidapi (maybe initialized before?)")
            }
        }
    }
}

impl Error for HidError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
