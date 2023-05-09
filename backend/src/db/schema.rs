// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (acc_id) {
        acc_id -> Int4,
        client_id -> Varchar,
        balance -> Float4,
        acc_activation_date -> Timestamp,
    }
}

diesel::table! {
    clients (client_id) {
        client_id -> Varchar,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        date_of_birth -> Timestamp,
    }
}

diesel::table! {
    transactions (id) {
        id -> Varchar,
        sender_id -> Varchar,
        receiver_id -> Varchar,
        amount -> Float4,
        withdrawal_time -> Timestamp,
    }
}

diesel::joinable!(accounts -> clients (client_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    clients,
    transactions,
);
