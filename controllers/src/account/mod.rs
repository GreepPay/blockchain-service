use crate::account::form::form::{
    ActivateAccountForm, CreateAccountForm, GetAccountByStellarAddressForm, GetManyAccountsForm,
    GetSingleAccountForm, SoftDeleteAccountForm, UpdateAccountForm,
};
use models::common::Pagination;
use models::models::Account;
use rocket::form::Form;
use services::account::account::{
    activate_account, create_account, get_account, get_account_by_stellar_address,
    get_many_accounts, soft_delete_account, update_account,
};

pub mod form;

// Get single account
pub async fn get_single_account_controller(
    data: Form<GetSingleAccountForm<'_>>,
) -> Result<Account, Box<dyn std::error::Error>> {
    Ok(get_account(data.account_id).await?)
}

// Create account
pub async fn create_account_controller(
    data: Form<CreateAccountForm<'_>>,
) -> Result<Account, Box<dyn std::error::Error>> {
    Ok(create_account(data.status, data.account_type).await?)
}

// Activate account
pub async fn activate_account_controller(
    data: Form<ActivateAccountForm<'_>>,
) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(activate_account(data.account_id).await?)
}

// Update account
pub async fn update_account_controller(
    data: Form<UpdateAccountForm<'_>>,
) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(update_account(data.account_id, data.status).await?)
}

// Soft delete account
pub async fn soft_delete_account_controller(
    data: Form<SoftDeleteAccountForm<'_>>,
) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(soft_delete_account(data.account_id).await?)
}

// Get many accounts
pub async fn get_many_accounts_controller(
    data: Form<GetManyAccountsForm>,
) -> Result<Pagination<Account>, Box<dyn std::error::Error>> {
    Ok(get_many_accounts(data.page).await?)
}

// Get account by stellar address
pub async fn get_account_by_stellar_address_controller(
    data: Form<GetAccountByStellarAddressForm<'_>>,
) -> Result<Account, Box<dyn std::error::Error>> {
    Ok(get_account_by_stellar_address(data.stellar_address).await?)
}
