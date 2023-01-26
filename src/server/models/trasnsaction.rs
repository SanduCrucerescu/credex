struct Transaction {
    id: String,
    sender_id: String,
    receiver_id: String,
    amount: f64,
    created_at: DateTime<Utc>,
    received_at: DateTime<Utc>,
}
