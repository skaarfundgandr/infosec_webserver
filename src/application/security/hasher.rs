use crate::domain::errors::hashing::HashingError;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use tokio::task;

pub async fn hash_password(password: &str) -> Result<String, HashingError> {
    let password = password.to_owned();
    let argon2 = Argon2::default();

    task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);

        match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(hash) => Ok(hash.to_string()),
            Err(_) => Err(HashingError::HashError),
        }
    })
    .await
    .map_err(|_| HashingError::HashError)?
}

pub async fn verify_password(password: &str, hash: &str) -> Result<bool, HashingError> {
    let password = password.to_owned();
    let hash = hash.to_owned();
    let argon2 = Argon2::default();

    task::spawn_blocking(move || {
        let parsed_hash = match PasswordHash::new(&hash) {
            Ok(hash) => hash,
            Err(_) => return Err(HashingError::ParsingError),
        };

        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(_) => Err(HashingError::VerificationError),
        }
    })
        .await
        .map_err(|_| HashingError::VerificationError)?
}