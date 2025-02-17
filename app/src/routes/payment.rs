pub mod payment {
    use controllers::api::api::{failure, success, ApiResponse};
    use controllers::payment::form::{
        EstablishTrustlineForm, SendNativePaymentForm, SendNonNativePaymentForm,
    };
    use controllers::payment::{
        establish_trustline_for_non_native_asset_controller, send_native_payment_controller,
        send_non_native_payment_controller,
    };
    use rocket::form::Form;
    use rocket::Route;
    use rocket::{get, http::Status, post, response::status, serde::json::Json};

    #[post("/trustline", data = "<form>")]
    pub async fn establish_trustline_for_non_native_asset(
        form: Form<EstablishTrustlineForm<'_>>,
    ) -> Result<status::Custom<Json<ApiResponse<bool>>>, status::Custom<Json<ApiResponse<()>>>>
    {
        let result = establish_trustline_for_non_native_asset_controller(form)
            .await
            .map_err(|e| {
                eprintln!("Error establishing trustline: {:?}", e);
                failure("Failed to establish trustline", Status::InternalServerError)
            })?;

        Ok(success(
            "Trustline established successfully",
            result,
            Status::Ok,
        ))
    }

    #[post("/native", data = "<form>")]
    pub async fn send_native_payment(
        form: Form<SendNativePaymentForm<'_>>,
    ) -> Result<status::Custom<Json<ApiResponse<bool>>>, status::Custom<Json<ApiResponse<()>>>>
    {
        let result = send_native_payment_controller(form).await.map_err(|e| {
            eprintln!("Error sending native payment: {:?}", e);
            failure("Failed to send native payment", Status::InternalServerError)
        })?;

        Ok(success(
            "Native payment sent successfully",
            result,
            Status::Ok,
        ))
    }

    #[post("/non-native", data = "<form>")]
    pub async fn send_non_native_payment(
        form: Form<SendNonNativePaymentForm<'_>>,
    ) -> Result<status::Custom<Json<ApiResponse<bool>>>, status::Custom<Json<ApiResponse<()>>>>
    {
        let result = send_non_native_payment_controller(form)
            .await
            .map_err(|e| {
                eprintln!("Error sending non-native payment: {:?}", e);
                failure(
                    "Failed to send non-native payment",
                    Status::InternalServerError,
                )
            })?;

        Ok(success(
            "Non-native payment sent successfully",
            result,
            Status::Ok,
        ))
    }
}
