// @generated automatically by Diesel CLI.

diesel::table! {
    users(id) {
        id -> Varchar,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        // created_at -> Nullable<Timestamptz>,
        // updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    transactions(id) {
        id -> Varchar,
        sender_id -> Varchar,
        receiver_id -> Varchar,
        amount -> Float,
        // created_at -> Nullable<Timestamptz>,
        // received_at -> Nullable<Timestamptz>,
    }
}

// diesel::joinable!(transactions -> users (sender_id));

diesel::allow_tables_to_appear_in_same_query!(users, transactions,);
