// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Varchar,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        balance -> Float8,
        date_of_birth -> Timestamp,
    }
}

diesel::table! {
    transactions (id) {
        id -> Varchar,
        sender_id -> Varchar,
        receiver_id -> Varchar,
        amount -> Float4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    clients,
    transactions,
);
