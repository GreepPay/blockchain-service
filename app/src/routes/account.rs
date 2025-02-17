pub mod account {
    use controllers::{
        account::form::form::{
            ActivateAccountForm, CreateAccountForm, GetAccountByStellarAddressForm,
            GetSingleAccountForm, SoftDeleteAccountForm, UpdateAccountForm,
        },
        api::api::{failure, success, ApiResponse},
    };
    use models::models::Account;
    use rocket::{form::Form, get, http::Status, post, response::status, serde::json::Json};

    #[get("/<account_id>")]
    pub async fn get_single_account(
        account_id: &str,
    ) -> Result<status::Custom<Json<ApiResponse<Account>>>, status::Custom<Json<ApiResponse<()>>>>
    {
        let get_single_account = GetSingleAccountForm { account_id };

        let single_account =
            controllers::account::get_single_account_controller(Form::from(get_single_account))
                .await
                .map_err(|_| {
                    failure("Failed to get single account", Status::InternalServerError)
                })?;

        Ok(success(
            "Account fetched successfully",
            single_account,
            Status::Ok,
        ))
    }

    #[post("/", data = "<form>")]
    pub async fn create_account<'r>(
        form: Form<CreateAccountForm<'r>>,
    ) -> Result<status::Custom<Json<ApiResponse<Account>>>, status::Custom<Json<ApiResponse<()>>>>
    {
        let new_account = controllers::account::create_account_controller(form)
            .await
            .map_err(|_| failure("Failed to create new account", Status::InternalServerError))?;

        Ok(success(
            "Account created successfully",
            new_account,
            Status::Ok,
        ))
    }

    #[post("/activate", data = "<form>")]
    pub async fn activate_account<'r>(
        form: Form<ActivateAccountForm<'r>>,
    ) -> Result<status::Custom<Json<ApiResponse<bool>>>, status::Custom<Json<ApiResponse<()>>>>
    {
        let activation_result = controllers::account::activate_account_controller(form)
            .await
            .map_err(|_| failure("Failed to activate account", Status::InternalServerError))?;

        Ok(success(
            "Account activated successfully",
            activation_result,
            Status::Ok,
        ))
    }

    #[post("/update", data = "<form>")]
    pub async fn update_account<'r>(
        form: Form<UpdateAccountForm<'r>>,
    ) -> Result<status::Custom<Json<ApiResponse<bool>>>, status::Custom<Json<ApiResponse<()>>>>
    {
        let update_result = controllers::account::update_account_controller(form)
            .await
            .map_err(|_| failure("Failed to update account", Status::InternalServerError))?;

        Ok(success(
            "Account updated successfully",
            update_result,
            Status::Ok,
        ))
    }

    #[post("/delete", data = "<form>")]
    pub async fn soft_delete_account<'r>(
        form: Form<SoftDeleteAccountForm<'r>>,
    ) -> Result<status::Custom<Json<ApiResponse<bool>>>, status::Custom<Json<ApiResponse<()>>>>
    {
        let delete_result = controllers::account::soft_delete_account_controller(form)
            .await
            .map_err(|_| failure("Failed to delete account", Status::InternalServerError))?;

        Ok(success(
            "Account deleted successfully",
            delete_result,
            Status::Ok,
        ))
    }

    #[get("/stellar?<stellar_address>")]
    pub async fn get_account_by_stellar_address(
        stellar_address: &str,
    ) -> Result<status::Custom<Json<ApiResponse<Account>>>, status::Custom<Json<ApiResponse<()>>>>
    {
        let get_account_by_stellar_address = GetAccountByStellarAddressForm { stellar_address };

        let account = controllers::account::get_account_by_stellar_address_controller(Form::from(
            get_account_by_stellar_address,
        ))
        .await
        .map_err(|_| {
            failure(
                "Failed to get account by stellar address",
                Status::InternalServerError,
            )
        })?;

        Ok(success("Account fetched successfully", account, Status::Ok))
    }
}
