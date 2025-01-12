use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TransactionResponse {
    pub _links: Links,
    pub id: String,
    pub paging_token: String,
    pub successful: bool,
    pub hash: String,
    pub ledger: i64,
    pub created_at: String,
    pub source_account: String,
    pub source_account_sequence: String,
    pub fee_account: String,
    pub fee_charged: String,
    pub max_fee: String,
    pub operation_count: i32,
    pub envelope_xdr: String,
    pub result_xdr: String,
    pub fee_meta_xdr: String,
    pub memo_type: String,
    pub signatures: Vec<String>,
    pub preconditions: Preconditions,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    pub self_link: HrefWrapper,
    pub account: HrefWrapper,
    pub ledger: HrefWrapper,
    pub operations: HrefTemplated,
    pub effects: HrefTemplated,
    pub precedes: HrefWrapper,
    pub succeeds: HrefWrapper,
    pub transaction: HrefWrapper,
}

#[derive(Debug, Deserialize)]
pub struct HrefWrapper {
    pub href: String,
}

#[derive(Debug, Deserialize)]
pub struct HrefTemplated {
    pub href: String,
    pub templated: bool,
}

#[derive(Debug, Deserialize)]
pub struct Preconditions {
    pub timebounds: TimeBounds,
}

#[derive(Debug, Deserialize)]
pub struct TimeBounds {
    pub min_time: String,
}
