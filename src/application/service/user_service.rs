use crate::application::repository::user_repo::UserRepo;
use crate::application::security::hasher;
use crate::domain::errors::service::UserServiceError;
use crate::domain::models::user::NewUser;

pub struct UserService;

impl UserService {
    pub async fn create_user(name: &str, password: &str) -> Result<(), UserServiceError> {
        let password_hash = hasher::hash_password(password)
            .await
            .map_err(|_| UserServiceError::UserCreationError)?;

        let new_user = NewUser {
            name,
            password_hash: &password_hash,
        };

        UserRepo::create(new_user)
            .await
            .map_err(|_| UserServiceError::UserCreationError)?;

        Ok(())
    }
}
