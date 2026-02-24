use crate::domain::user::User;
use crate::infrastructure::database::Database;
use diesel::prelude::*;
use diesel::result;
use diesel_async::RunQueryDsl;

pub struct UserRepo;

impl UserRepo {
    pub async fn get_by_id(id: i32) -> Result<Option<User>, result::Error> {
        use crate::domain::schema::users::dsl::{user_id, users};

        let db = Database::new();

        let mut conn = db.get_connection().await.map_err(|e| {
            result::Error::DatabaseError(
                result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        match users.filter(user_id.eq(id)).first::<User>(&mut conn).await {
            Ok(user) => Ok(Some(user)),
            Err(result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
