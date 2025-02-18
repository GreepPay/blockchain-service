use crate::payment::form::form::{
    EstablishTrustlineForm, SendNativePaymentForm, SendNonNativePaymentForm,
};
use rocket::form::Form;
use services::payment::payment::{
    establish_trustline_for_non_native_asset, send_native_payment, send_non_native_payment,
};

pub mod form;

/// Establish a trustline for a non-native asset.
pub async fn establish_trustline_for_non_native_asset_controller<'r>(
    form: Form<EstablishTrustlineForm<'r>>,
) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(establish_trustline_for_non_native_asset(
        form.account_id.to_string(),
        form.asset_code,
        form.asset_issuer,
    )
    .await?)
}

/// Send a native payment (XLM).
pub async fn send_native_payment_controller<'r>(
    form: Form<SendNativePaymentForm<'r>>,
) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(send_native_payment(
        form.sender_account_id.to_string(),
        form.receiver_public_key,
        form.amount,
    )
    .await?)
}

/// Send a non-native payment.
pub async fn send_non_native_payment_controller<'r>(
    form: Form<SendNonNativePaymentForm<'r>>,
) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(send_non_native_payment(
        form.sender_account_id.to_string(),
        form.receiver_public_key,
        form.asset_code,
        form.asset_issuer,
        form.amount,
    )
    .await?)
}
