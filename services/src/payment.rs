pub mod payment {
    use crate::common::common::get_account_from_id;
    use crate::common::common::get_chain_network;
    use crate::common::common::save_chain_transaction;
    use anyhow::Error;
    use bigdecimal::BigDecimal;
    use helpers::stellar_chain::StellarChain;
    use reqwest::Response;
    use stellar_base::asset::{Asset, CreditAsset};
    use stellar_base::PublicKey;

    /// Establish a trustline for a non-native asset.

    pub async fn establish_trustline_for_non_native_asset(
        account_id: String,
        asset_code: &str,
        asset_issuer: &str,
    ) -> Result<Response, Error> {
        let network = get_chain_network()?;
        let stellar_chain = StellarChain::new(std::env::var("STELLAR_HORIZON_URL")?, network);

        let (account, keypair) = get_account_from_id(account_id).await?;

        // Create the custom asset
        let asset = Asset::Credit(CreditAsset::new(
            asset_code.to_string(),
            PublicKey::from_account_id(asset_issuer)?,
        )?);

        // Establish the trustline for the custom asset
        stellar_chain
            .establish_trustline_for_asset(keypair, asset)
            .await
    }

    // Sends a native payment (XLM) and saves the transaction to the database.
    pub async fn send_native_payment(
        sender_account_id: String,
        receiver_public_key: &str,
        amount: u64,
    ) -> Result<Response, Error> {
        send_payment(
            sender_account_id,
            receiver_public_key,
            Asset::Native, // Native asset (XLM)
            "XLM".to_string(),
            amount,
        )
        .await
    }

    // Sends a non-native payment and saves the transaction to the database.
    pub async fn send_non_native_payment(
        sender_account_id: String,
        receiver_public_key: &str,
        asset_code: &str,
        asset_issuer: &str,
        amount: u64,
    ) -> Result<Response, Error> {
        // Create the custom asset
        let asset = Asset::Credit(CreditAsset::new(
            asset_code.to_string(),
            PublicKey::from_account_id(asset_issuer)?,
        )?);

        send_payment(
            sender_account_id,
            receiver_public_key,
            asset,
            asset_code.to_string(),
            amount,
        )
        .await
    }

    // function to send a payment and save the transaction to the database.

    async fn send_payment(
        sender_account_id: String,
        receiver_public_key: &str,
        asset: Asset, // Can be Native or Credit
        asset_code: String,
        amount: u64,
    ) -> Result<Response, Error> {
        let network = get_chain_network()?;
        let stellar_chain = StellarChain::new(std::env::var("STELLAR_HORIZON_URL")?, network);

        let (sender_account, sender_keypair) = get_account_from_id(sender_account_id).await?;

        // Send the payment
        let response = stellar_chain
            .send_asset(
                sender_keypair,
                receiver_public_key.to_string(),
                asset,
                amount,
            )
            .await?;

        // Save the transaction to the database
        save_chain_transaction(
            response,
            PublicKey::from_account_id(&sender_account.stellar_address)?,
            PublicKey::from_account_id(receiver_public_key)?,
            asset_code,
            BigDecimal::from(amount),
        )
        .await?;

        Ok(response)
    }
}
