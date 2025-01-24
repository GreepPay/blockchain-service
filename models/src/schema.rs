// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Uuid,
        stellar_address -> Text,
        account_type -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        status -> Text,
    }
}

diesel::table! {
    encrypted_keys (id) {
        id -> Uuid,
        account_id -> Uuid,
        encrypted_key -> Byte,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tokens (id) {
        id -> Uuid,
        asset_code -> Text,
        issuer_account_id -> Nullable<Uuid>,
        total_supply -> Nullable<Numeric>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    transaction_errors (id) {
        id -> Uuid,
        transaction_id -> Nullable<Uuid>,
        error_code -> Text,
        error_message -> Text,
        occurred_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    transactions (id) {
        id -> Uuid,
        source_account_id -> Nullable<Uuid>,
        destination_account_id -> Nullable<Uuid>,
        transaction_hash -> Text,
        amount -> Numeric,
        asset_code -> Text,
        memo -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        status -> Text,
    }
}

diesel::table! {
    trustlines (id) {
        id -> Uuid,
        account_id -> Nullable<Uuid>,
        asset_code -> Text,
        asset_issuer -> Text,
        trust_limit -> Nullable<Numeric>,
        created_at -> Nullable<Timestamp>,
        status -> Text,
    }
}

diesel::joinable!(encrypted_keys -> accounts (account_id));
diesel::joinable!(tokens -> accounts (issuer_account_id));
diesel::joinable!(transaction_errors -> transactions (transaction_id));
diesel::joinable!(trustlines -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    encrypted_keys,
    tokens,
    transaction_errors,
    transactions,
    trustlines,
);
