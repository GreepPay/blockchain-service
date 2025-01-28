use form::form::CreateAccountForm;
use models::models::Account;
use rocket::form::Form;

use crate::account::form::form::GetSingleAccountForm;
pub mod form;


// Get single account
pub async fn get_single_account(
    data: Form<GetSingleAccountForm<'_>>,
) -> Result<Account, Box<dyn std::error::Error>> {
    Ok(services::account::account::get_account(data.account_id).await?)
}

// Create account
pub async fn create_account(
    data: Form<CreateAccountForm<'_>>,
) -> Result<Account, Box<dyn std::error::Error>> {
    Ok(services::account::account::create_account(data.status, data.account_type).await?)
}
