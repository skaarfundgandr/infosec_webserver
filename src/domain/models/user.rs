use crate::domain::models::schema::*;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Identifiable, Serialize, PartialEq, Eq, QueryableByName)]
#[diesel(table_name = users)]
#[diesel(primary_key(user_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub password_hash: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser<'a> {
    pub name: Option<&'a str>,
    pub password_hash: Option<&'a str>,
}
