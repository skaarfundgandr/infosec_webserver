use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum HashingError {
    HashError,
    VerificationError,
    ParsingError,
}

impl Display for HashingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HashingError::HashError => write!(f, "Error hashing password"),
            HashingError::VerificationError => write!(f, "Error verifying password"),
            HashingError::ParsingError => write!(f, "Error parsing password hash"),
        }
    }
}

impl std::error::Error for HashingError {}
