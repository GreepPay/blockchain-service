pub mod common {

    use anyhow::{Error, Ok};
    use bigdecimal::BigDecimal;
    use diesel_async::RunQueryDsl;
    use helpers::common::decrypt_private_key;
    use models::models::AccountWithKey;
    use models::{common::establish_connection, models::NewTransaction, schema};
    use stellar_base::{Network, PublicKey};
    use reqwest::Response;
    use stellar_sdk::types::SubmitTransactionResponse;
    use stellar_sdk::Keypair;
    use uuid::Uuid;
    use diesel::QueryDsl;
    use diesel::ExpressionMethods;
    use diesel::JoinOnDsl;

    /// Returns the Stellar network (testnet or public) based on the CHAIN_ENVIRONMENT environment variable
    /// 
    /// # Returns
    /// * `Ok(Network)` - The configured Stellar network
    /// * `Err(Error)` - If the chain environment is invalid
    pub fn get_chain_network() -> Result<Network, Error> {
        let chain_environment = std::env::var("CHAIN_ENVIRONMENT").unwrap();
        match chain_environment {
            ref chain_env if chain_env == "testnet" => Ok(Network::new_test()),
            ref chain_env if chain_env == "public" => Ok(Network::new_public()),
            _ => return Err(anyhow::anyhow!("Invalid chain environment")),
        }
    }

    /// Saves a blockchain transaction to the database after it has been submitted to the network
    /// 
    /// # Arguments
    /// * `response` - The HTTP response from submitting the transaction
    /// * `source_account` - The public key of the sending account
    /// * `destination_account` - The public key of the receiving account
    /// * `asset_code` - The code/symbol of the asset being transferred
    /// * `amount` - The amount of the asset being transferred
    /// 
    /// # Returns
    /// * `Ok(())` - If the transaction was successfully saved
    /// * `Err(Error)` - If there was an error saving the transaction
    pub async fn save_chain_transaction(response: Response, source_account: PublicKey, destination_account: PublicKey, asset_code: String, amount: BigDecimal) -> Result<(), Error> {
        let transaction = response.json::<SubmitTransactionResponse>().await.unwrap();

        let mut db_connection = establish_connection().await.unwrap();

        let hash = transaction.hash.to_string();
        let memo = transaction.memo.unwrap_or_default().to_string();

        let new_transaction = NewTransaction {
            id: Uuid::new_v4(),
            // id: Uuid::new_v4(),
            source_account_id: Uuid::parse_str(source_account.to_string().as_str()).unwrap(),
            destination_account_id: Uuid::parse_str(destination_account.to_string().as_str()).unwrap(),
            transaction_hash: hash.as_str(),
            amount: Some(amount),
            asset_code: asset_code.as_str(),
            memo: Some(memo.as_str()),
            status: if transaction.successful { "success" } else { "failed" },
            created_at: {
                let created_at = chrono::DateTime::parse_from_rfc3339(&transaction.created_at).unwrap();
                Some(created_at.naive_utc())
            },
        };

        diesel::insert_into(schema::transactions::table)
            .values(&new_transaction)
            .execute(&mut db_connection)
            .await?;
        
        Ok(())
    }

    pub async fn get_account_from_id(account_id: String) -> Result<(AccountWithKey, Keypair), Error> {

        let mut db_connection = establish_connection().await.unwrap();

        let account_uuid = Uuid::parse_str(account_id.as_str()).unwrap();

        // Get account
        let account =
            models::schema::accounts::table
                .inner_join(models::schema::encrypted_keys::table.on(
                    models::schema::encrypted_keys::account_id.eq(models::schema::accounts::id),
                ))
                .filter(models::schema::accounts::id.eq(account_uuid))
                .select((
                    models::schema::accounts::id,
                    models::schema::accounts::stellar_address,
                    models::schema::accounts::account_type,
                    models::schema::accounts::created_at,
                    models::schema::accounts::updated_at,
                    models::schema::accounts::status,
                    models::schema::encrypted_keys::encrypted_key,
                ))
                .first::<AccountWithKey>(&mut db_connection)
                .await?;

        let decrypted_key = decrypt_private_key(&account.encrypted_key).unwrap();

        let account_keypair =
            Keypair::from_secret_key(std::str::from_utf8(&decrypted_key).unwrap()).unwrap();

        Ok((account, account_keypair))
    }
}