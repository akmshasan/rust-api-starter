// @generated automatically by Diesel CLI.

diesel::table! {
    fruits (id) {
        id -> Int8,
        name -> Varchar,
        color -> Varchar,
        price -> Float8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
