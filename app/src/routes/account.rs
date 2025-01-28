pub mod account {
    use controllers::{account::form::form::{CreateAccountForm, GetSingleAccountForm}, api::api::{failure, success, ApiResponse}};
    use models::models::Account;
    use rocket::{
       form::Form, get, http::Status, post, response::status, serde::json::Json
    };

    #[get("/<account_id>")]
    pub async fn get_single_account(
        account_id: &str,
    ) -> Result<status::Custom<Json<ApiResponse<Account>>>, status::Custom<Json<ApiResponse<()>>>>
    {

        let get_single_account = GetSingleAccountForm {
            account_id: account_id,
        };
        
        let single_account = controllers::account::get_single_account(Form::<GetSingleAccountForm>::from(get_single_account))
            .await
            .map_err(|_| failure("Failed to get single account", Status::InternalServerError))?;

        Ok(success("Account fetched successfully", single_account, Status::Ok))
    }

    #[post("/", data = "<form>")]
    pub async fn create_account<'r>(
        form: Form<CreateAccountForm<'r>>,
    ) -> Result<status::Custom<Json<ApiResponse<Account>>>, status::Custom<Json<ApiResponse<()>>>> {
        
        let new_account = controllers::account::create_account(form)
            .await
            .map_err(|_| failure("Failed to create new account", Status::InternalServerError))?;
        
        Ok(success("Account created successfully", new_account, Status::Ok))
    }
}
