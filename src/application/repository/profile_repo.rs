use crate::domain::models::user_profile::{NewUserProfile, UpdateUserProfile, UserProfile};
use crate::infrastructure::database::Database;
use diesel::prelude::*;
use diesel::result;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};

pub struct ProfileRepo;

impl ProfileRepo {
    pub async fn get_by_user_id(id: i32) -> Result<Option<UserProfile>, result::Error> {
        use crate::domain::models::schema::user_profiles::dsl::{user_id, user_profiles};

        let db = Database::new();
        let mut conn = db.get_connection().await.map_err(|e| {
            result::Error::DatabaseError(
                result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        match user_profiles
            .filter(user_id.eq(id))
            .first::<UserProfile>(&mut conn)
            .await
        {
            Ok(profile) => Ok(Some(profile)),
            Err(result::Error::NotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn create(new_profile: NewUserProfile<'_>) -> Result<(), result::Error> {
        use crate::domain::models::schema::user_profiles::dsl::user_profiles;

        let db = Database::new();
        let mut conn = db.get_connection().await.map_err(|e| {
            result::Error::DatabaseError(
                result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        match conn
            .transaction(|connection| {
                async move {
                    diesel::insert_into(user_profiles)
                        .values(&new_profile)
                        .execute(connection)
                        .await?;

                    Ok(())
                }
                .scope_boxed()
            })
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn update(id: i32, changeset: UpdateUserProfile<'_>) -> Result<(), result::Error> {
        use crate::domain::models::schema::user_profiles::dsl::{user_id, user_profiles};

        let db = Database::new();
        let mut conn = db.get_connection().await.map_err(|e| {
            result::Error::DatabaseError(
                result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;

        match conn
            .transaction(|connection| {
                async move {
                    diesel::update(user_profiles.filter(user_id.eq(id)))
                        .set(&changeset)
                        .execute(connection)
                        .await?;

                    Ok(())
                }
                .scope_boxed()
            })
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
