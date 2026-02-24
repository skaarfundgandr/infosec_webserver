use diesel::prelude::*;
use crate::domain::schema::*;

#[derive(Queryable, Selectable, Identifiable, PartialEq, Eq)]
#[diesel(table_name = users)]
#[diesel(primary_key(user_id))]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(treat_none_as_null = true)]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}
