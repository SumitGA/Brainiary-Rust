// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        hashed_password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
