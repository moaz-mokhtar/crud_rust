// @generated automatically by Diesel CLI.

diesel::table! {
    drivers (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
    }
}
