use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum UserServiceError {
    UserCreationError,
}

impl Display for UserServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserServiceError::UserCreationError => write!(f, "Error creating user"),
        }
    }
}

impl Error for UserServiceError {}
