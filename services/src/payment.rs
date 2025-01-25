pub mod payment {
    use anyhow::Error;
    use bigdecimal::BigDecimal;
    use chrono::Utc;
    use diesel::insert_into;
    use diesel::prelude::*;
    use helpers::stellar_chain::StellarChain;
    use models::common::establish_connection;
    use models::models::Transaction;
    use models::schema::transactions;
    use reqwest::Response;
    use stellar_base::asset::{Asset, CreditAsset};
    use stellar_sdk::Keypair;
    use uuid::Uuid;

    use crate::common::common::get_account_from_id;
    use crate::common::common::get_chain_network;
    use crate::common::common::save_chain_transaction;

    // Use account ID 
    /// Establish a trustline for a non-native asset.
    pub async fn establish_trustline_for_non_native_asset(
        account_id: String,
        asset_code: &str,
        asset_issuer: &str,
    ) -> Result<Response, Error> {
        let network = get_chain_network()?;
        let stellar_chain = StellarChain::new(std::env::var("STELLAR_HORIZON_URL")?, network);

        let (account, keypair) = get_account_from_id(account_id).await?;

        let keypair = Keypair::from_secret_key(account_secret)?;

        let asset = CreditAsset::new(
            asset_code.to_string(),
            stellar_base::PublicKey::from_account_id(asset_issuer)?,
        )?;

        stellar_chain
            .establish_trustline_for_asset(keypair, Asset::Credit(asset))
            .await
    }

    // Use account ID to get the keypair which contains the secret key
    /// Sends a native payment (XLM) and saves the transaction to the database.
    pub async fn send_native_payment(
        sender_secret: &str,
        receiver_public_key: &str,
        amount: u64,
        status: &str,
    ) -> Result<Response, Error> {
        send_payment(
            sender_secret,
            receiver_public_key,
            Asset::Native,
            "XLM".to_string(),
            amount,
            status,
        )
        .await
    }

    // Use account ID to get the keypair which contains the secret key
    /// Sends a non-native payment and saves the transaction to the database.
    pub async fn send_non_native_payment(
        sender_secret: &str,
        receiver_public_key: &str,
        asset_code: &str,
        asset_issuer: &str,
        amount: u64,
        status: &str,
    ) -> Result<Response, Error> {
        let asset = Asset::Credit(CreditAsset::new(
            asset_code.to_string(),
            stellar_base::PublicKey::from_account_id(asset_issuer)?,
        )?);

        send_payment(
            sender_secret,
            receiver_public_key,
            asset,
            asset_code.to_string(),
            amount,
            status,
        )
        .await
    }
    
    // User the save_chain_transaction function to save the transaction to the database
    /// Helper function to send a payment and save the transaction to the database.
    async fn send_payment(
        sender_secret: &str,
        receiver_public_key: &str,
        asset: Asset,
        asset_code: String,
        amount: u64,
        status: &str,
    ) -> Result<Response, Error> {
        let network = get_chain_network()?;
        let stellar_chain = StellarChain::new(std::env::var("STELLAR_HORIZON_URL")?, network);

        let sender_keypair = Keypair::from_secret_key(sender_secret)?;

        // Send the payment
        let response = stellar_chain
            .send_asset(
                sender_keypair.clone(),
                receiver_public_key.to_string(),
                asset,
                amount,
            )
            .await?;

        // Save the transaction to the database
        // save_chain_transaction(response, sender_keypair, receiver_public_key, asset_code, amount).await?;
        // let mut db_connection = establish_connection().await?;

        // let new_transaction = Transaction {
        //     id: Uuid::new_v4(),
        //     source_account_id: Uuid::parse_str(sender_keypair.public_key().as_str())?,
        //     destination_account_id: Uuid::parse_str(receiver_public_key)?,
        //     transaction_hash: response.hash.to_string(), // Ensure this matches the response structure
        //     amount: BigDecimal::from(amount),
        //     asset_code,
        //     memo: None,
        //     created_at: Some(Utc::now().naive_utc()),
        //     status: status.to_string(),
        // };

        // insert_into(transactions::table)
        //     .values(&new_transaction)
        //     .execute(&mut db_connection)
        //     .await?;

        Ok(response)
    }
}
