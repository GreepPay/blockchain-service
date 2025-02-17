pub mod form {
    use rocket::form::FromForm;

    #[derive(FromForm)]
    pub struct GetSingleAccountForm<'r> {
        pub account_id: &'r str,
    }

    #[derive(FromForm)]
    pub struct CreateAccountForm<'r> {
        pub status: &'r str,
        pub account_type: &'r str,
    }

    #[derive(FromForm)]
    pub struct ActivateAccountForm<'r> {
        pub account_id: &'r str,
    }

    #[derive(FromForm)]
    pub struct UpdateAccountForm<'r> {
        pub account_id: &'r str,
        pub status: &'r str,
    }

    #[derive(FromForm)]
    pub struct SoftDeleteAccountForm<'r> {
        pub account_id: &'r str,
    }

    #[derive(FromForm)]
    pub struct GetManyAccountsForm {
        pub page: i64,
    }

    #[derive(FromForm)]
    pub struct GetAccountByStellarAddressForm<'r> {
        pub stellar_address: &'r str,
    }
}
