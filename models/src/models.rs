use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::*;

/// Represents an account in the system.
#[derive(Queryable, Serialize, Deserialize, Selectable, Insertable)]
#[diesel(table_name = accounts)]

pub struct Account {
    pub id: Uuid,
    pub stellar_address: String,
    pub account_type: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub status: String,
}

/// Represents a trustline for a specific asset.
#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = trustlines)]
pub struct Trustline {
    pub id: Uuid,
    pub account_id: Uuid,
    pub asset_code: String,
    pub asset_issuer: String,
    pub trust_limit: f64,
    pub created_at: NaiveDateTime,
    pub status: String,
}

#[derive(Insertable)]
#[diesel(table_name = trustlines)]
pub struct NewTrustline<'a> {
    pub account_id: Uuid,
    pub asset_code: &'a str,
    pub asset_issuer: &'a str,
    pub trust_limit: Option<BigDecimal>,
    pub status: &'a str,
}

/// Represents a transaction in the blockchain system.
#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: Uuid,
    pub source_account_id: Uuid,
    pub destination_account_id: Uuid,
    pub transaction_hash: String,
    pub amount: f64,
    pub asset_code: String,
    pub memo: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub status: String,
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction<'a> {
    pub id: Uuid,
    pub source_account_id: Uuid,
    pub destination_account_id: Uuid,
    pub transaction_hash: &'a str,
    pub amount: Option<BigDecimal>,
    pub asset_code: &'a str,
    pub memo: Option<&'a str>,
    pub created_at: Option<NaiveDateTime>,
    pub status: &'a str,
}

/// Represents an error that occurred during a transaction.
#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = transaction_errors)]
pub struct TransactionError {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub error_code: String,
    pub error_message: String,
    pub occurred_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = transaction_errors)]
pub struct NewTransactionError<'a> {
    pub transaction_id: Uuid,
    pub error_code: &'a str,
    pub error_message: &'a str,
}

/// Represents an encrypted private key.
#[derive(Queryable, Serialize, Deserialize, Selectable, Insertable)]
#[diesel(table_name = encrypted_keys)]
#[diesel(belongs_to(Account, foreign_key = account_id))]
pub struct EncryptedKey {
    pub id: Uuid,
    pub account_id: Uuid,
    pub encrypted_key: Vec<u8>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Queryable)]
pub struct AccountWithKey {
    pub id: uuid::Uuid,
    pub stellar_address: String,
    pub account_type: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub status: String,
    pub encrypted_key: Vec<u8>,
}
