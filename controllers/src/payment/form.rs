pub mod form {
    use rocket::form::FromForm;

    #[derive(FromForm)]
    pub struct EstablishTrustlineForm<'r> {
        pub account_id: &'r str,
        pub asset_code: &'r str,
        pub asset_issuer: &'r str,
    }

    #[derive(FromForm)]
    pub struct SendNativePaymentForm<'r> {
        pub sender_account_id: &'r str,
        pub receiver_public_key: &'r str,
        pub amount: u64,
    }

    #[derive(FromForm)]
    pub struct SendNonNativePaymentForm<'r> {
        pub sender_account_id: &'r str,
        pub receiver_public_key: &'r str,
        pub asset_code: &'r str,
        pub asset_issuer: &'r str,
        pub amount: u64,
    }
}
