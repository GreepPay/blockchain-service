/// Account management module that handles blockchain account operations including creation,
/// activation, retrieval, and updates. This module primarily works with Stellar blockchain
/// accounts and their corresponding database records.
pub mod account {

    // Create account
    // Activate account
    // Get account
    // Get many accounts
    // Update account
    // Soft delete account

    use anyhow::{Error, Ok};
    use bigdecimal::BigDecimal;
    use diesel::ExpressionMethods;
    use diesel::JoinOnDsl;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    use helpers::{
        common::{decrypt_private_key, encrypt_private_key},
        stellar_chain::StellarChain,
    };
    use models::common::Paginate;
    use models::common::Pagination;
    use models::schema::accounts;
    use models::{
        common::establish_connection,
        models::{Account, AccountWithKey, EncryptedKey},
    };
    use stellar_sdk::Keypair;
    use uuid::Uuid;

    use crate::common::common;

    /// Retrieves an account by its unique identifier
    ///
    /// # Arguments
    /// * `account_id` - A string slice containing the UUID of the account
    ///
    /// # Returns
    /// * `Result<Account, Error>` - The account if found, or an error if not
    // Get account
    pub async fn get_account(account_id: &str) -> Result<Account, Error> {
        let mut db_connection = establish_connection().await.unwrap();
        let account_uuid = Uuid::parse_str(account_id).unwrap();
        let account = models::schema::accounts::table
            .find(account_uuid)
            .first(&mut db_connection)
            .await?;

        Ok(account)
    }

    /// Retrieves an account by its Stellar blockchain address
    ///
    /// # Arguments
    /// * `stellar_address` - A string slice containing the Stellar public key
    ///
    /// # Returns
    /// * `Result<Account, Error>` - The account if found, or an error if not
    pub async fn get_account_by_stellar_address(stellar_address: &str) -> Result<Account, Error> {
        let mut db_connection = establish_connection().await.unwrap();
        let account = models::schema::accounts::table
            .filter(models::schema::accounts::stellar_address.eq(stellar_address))
            .first(&mut db_connection)
            .await?;
        Ok(account)
    }

    /// Retrieves a paginated list of accounts
    ///
    /// # Arguments
    /// * `page` - The page number to retrieve
    ///
    /// # Returns
    /// * `Result<Pagination<Account>, Error>` - A paginated result containing accounts
    pub async fn get_many_accounts(page: i64) -> Result<Pagination<Account>, Error> {
        let mut db_connection = establish_connection().await.unwrap();

        let query = accounts::table.select(accounts::all_columns).into_boxed();

        let (data, total_pages, total_records, per_page) = query
            .paginate(page)
            .per_page(10)
            .load_and_count_pages::<Account>(&mut db_connection)
            .await?;

        Ok(Pagination {
            data,
            total_pages,
            total_records,
            page,
            per_page,
        })
    }

    /// Creates a new blockchain account with corresponding database records
    ///
    /// # Arguments
    /// * `status` - Initial status of the account
    /// * `account_type` - Type of account to create
    ///
    /// # Returns
    /// * `Result<Account, Error>` - The newly created account or an error
    pub async fn create_account(status: &str, account_type: &str) -> Result<Account, Error> {
        let network = common::get_chain_network().unwrap();

        let stellar_chain =
            StellarChain::new(std::env::var("STELLAR_HORIZON_URL").unwrap(), network);
        let new_stellar_account = stellar_chain.create_new_account().unwrap();

        let mut db_connection = establish_connection().await.unwrap();

        let new_account = Account {
            id: Uuid::new_v4(),
            created_at: Some(chrono::Utc::now().naive_utc()),
            updated_at: Some(chrono::Utc::now().naive_utc()),
            status: status.to_string(),
            account_type: account_type.to_string(),
            stellar_address: new_stellar_account.public_key.as_str().to_string(),
        };

        let account: Account = diesel::insert_into(models::schema::accounts::table)
            .values(&new_account)
            .returning(models::schema::accounts::all_columns)
            .get_result(&mut db_connection)
            .await?;

        // Save encrypted key  //shadowing right?
        let new_encrypted_key: Vec<u8> =
            encrypt_private_key(new_stellar_account.secret_key.as_str().as_bytes()).unwrap();

        let new_encrypted_key = EncryptedKey {
            id: Uuid::new_v4(),
            account_id: account.id,
            encrypted_key: new_encrypted_key,
            created_at: Some(chrono::Utc::now().naive_utc()),
        };

        diesel::insert_into(models::schema::encrypted_keys::table)
            .values(&new_encrypted_key)
            .execute(&mut db_connection)
            .await?;

        Ok(account)
    }

    /// Activates an existing account on the blockchain
    ///
    /// # Arguments
    /// * `account_id` - A string slice containing the UUID of the account to activate
    ///
    /// # Returns
    /// * `Result<bool, Error>` - True if activation successful, error otherwise
    ///
    /// # Errors
    /// Returns an error if the account is already active
    pub async fn activate_account(account_id: &str) -> Result<bool, Error> {
        let mut db_connection = establish_connection().await.unwrap();

        let account_uuid = Uuid::parse_str(account_id).unwrap();

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

        if account.status == "active" {
            return Err(anyhow::anyhow!("Account already active"));
        }

        // Activate account on chain
        let network = common::get_chain_network().unwrap();
        let stellar_chain =
            StellarChain::new(std::env::var("STELLAR_HORIZON_URL").unwrap(), network);

        let decrypted_key = decrypt_private_key(&account.encrypted_key).unwrap();

        let account_keypair =
            Keypair::from_secret_key(std::str::from_utf8(&decrypted_key).unwrap()).unwrap();

        let (activation_response, funding_account, new_account, amount) =
            stellar_chain.activate_account(account_keypair).await?;

        let amount_to_bigint = amount.to_i64().to_string().parse::<BigDecimal>().unwrap();

        common::save_chain_transaction(
            activation_response,
            funding_account,
            new_account,
            "XLM".to_string(),
            amount_to_bigint,
        )
        .await?;

        // Update account status
        diesel::update(models::schema::accounts::table)
            .filter(models::schema::accounts::id.eq(account_uuid))
            .set(models::schema::accounts::status.eq("active"))
            .execute(&mut db_connection)
            .await?;

        Ok(true)
    }

    /// Updates an account's status
    ///
    /// # Arguments
    /// * `account_id` - A string slice containing the UUID of the account
    /// * `status` - The new status to set
    ///
    /// # Returns
    /// * `Result<bool, Error>` - True if update successful, error otherwise
    pub async fn update_account(account_id: &str, status: &str) -> Result<bool, Error> {
        let mut db_connection = establish_connection().await.unwrap();
        let account_uuid = Uuid::parse_str(account_id).unwrap();

        diesel::update(models::schema::accounts::table)
            .filter(models::schema::accounts::id.eq(account_uuid))
            .set(models::schema::accounts::status.eq(status))
            .execute(&mut db_connection)
            .await?;

        Ok(true)
    }

    /// Performs a soft delete on an account by setting its status to "deleted"
    ///
    /// # Arguments
    /// * `account_id` - A string slice containing the UUID of the account to delete
    ///
    /// # Returns
    /// * `Result<bool, Error>` - True if deletion successful, error otherwise
    pub async fn soft_delete_account(account_id: &str) -> Result<bool, Error> {
        let mut db_connection = establish_connection().await.unwrap();
        let account_uuid = Uuid::parse_str(account_id).unwrap();

        diesel::update(models::schema::accounts::table)
            .filter(models::schema::accounts::id.eq(account_uuid))
            .set(models::schema::accounts::status.eq("closed"))
            .execute(&mut db_connection)
            .await?;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::account::*;
    // use mockall::predicate::*;
    // use models::models::Account;

    #[tokio::test]
    async fn test_get_account() {
        let account_id = "550e8400-e29b-41d4-a716-446655440000";
        let result = get_account(account_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_account_by_stellar_address() {
        let stellar_address = "GDJLBYYKMCXNVVNKFKNV6GKZW6PQTG2VHJNX7YX77MHXPKGKBEVW7PW2";
        let result = get_account_by_stellar_address(stellar_address).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_account() {
        let status = "pending";
        let account_type = "user";
        let result = create_account(status, account_type).await;
        assert!(result.is_ok());

        if let Ok(account) = result {
            assert_eq!(account.status, "pending");
            assert_eq!(account.account_type, "user");
            assert!(!account.stellar_address.is_empty());
        }
    }

    #[tokio::test]
    async fn test_activate_account() {
        let account_id = "550e8400-e29b-41d4-a716-446655440000";
        let result = activate_account(account_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_account() {
        let account_id = "550e8400-e29b-41d4-a716-446655440000";
        let new_status = "inactive";
        let result = update_account(account_id, new_status).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_soft_delete_account() {
        let account_id = "550e8400-e29b-41d4-a716-446655440000";
        let result = soft_delete_account(account_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_many_accounts() {
        let page = 1;
        let result = get_many_accounts(page).await;
        assert!(result.is_ok());

        if let Ok(pagination) = result {
            assert!(pagination.total_records >= 0);
            assert!(pagination.page == 1);
        }
    }
}
