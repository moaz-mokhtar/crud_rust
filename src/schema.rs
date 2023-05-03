// @generated automatically by Diesel CLI.

diesel::table! {
    drivers (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
    }
}
