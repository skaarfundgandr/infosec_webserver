// @generated automatically by Diesel CLI.

diesel::table! {
    user_profiles (user_id) {
        user_id -> Integer,
        bio -> Nullable<Text>,
        #[max_length = 255]
        avatar_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
    }
}

diesel::joinable!(user_profiles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(user_profiles, users,);
