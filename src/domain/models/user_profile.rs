use crate::domain::models::schema::*;
use crate::domain::models::user::User;
use diesel::prelude::*;

#[derive(Selectable, Queryable, Identifiable, Associations, PartialEq, Eq)]
#[diesel(table_name = user_profiles)]
#[diesel(primary_key(user_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserProfile {
    pub user_id: i32,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = user_profiles)]
pub struct NewUserProfile<'a> {
    pub user_id: i32,
    pub bio: Option<&'a str>,
    pub avatar_url: Option<&'a str>,
}

#[derive(AsChangeset)]
#[diesel(table_name = user_profiles)]
pub struct UpdateUserProfile<'a> {
    pub bio: Option<&'a str>,
    pub avatar_url: Option<&'a str>,
}
