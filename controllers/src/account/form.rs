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
}
