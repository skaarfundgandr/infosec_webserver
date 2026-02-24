// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}
