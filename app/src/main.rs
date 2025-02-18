#[macro_use]
extern crate rocket;
use app::routes::{account::account, payment::payment};
// use helpers::{asset_issuer::AssetIssuer, stellar_chain::StellarChain};
// use stellar_base::Network;
// use stellar_sdk::Keypair;

#[launch]
async fn rocket() -> _ {
    // Load env
    dotenv::dotenv().ok();

    // Initialize asset issuer Do this only once
    // let asset_issuer = AssetIssuer::new(
    //     "https://horizon-testnet.stellar.org".to_string(),
    //     std::env::var("ISSUER_SECRET_KEY").unwrap(),
    //     std::env::var("RECEIVER_SECRET_KEY").unwrap(),
    //     "GRP".to_string()
    // );

    // Create trustline
    // asset_issuer.create_trustline().await.unwrap();

    // Issue asset
    // asset_issuer.issue_asset().await.unwrap();

    // Create and activate account
    // let stellar_chain = StellarChain::new("https://horizon-testnet.stellar.org".to_string(), Network::new_test());

    // let account = stellar_chain.create_new_account().unwrap();

    // let account_keypair = Keypair::from_secret_key(account.secret_key.as_str()).unwrap();

    // stellar_chain.activate_account(account_keypair).await.unwrap();

    // Generate encryption key and iv. Use when generating a new key.
    // helpers::common::generate_encryption_key_and_iv();

    // Launch application
    rocket::build()
        .mount(
            "/v1/accounts",
            routes![
                account::get_single_account,
                account::create_account,
                account::activate_account,
                account::update_account,
                account::soft_delete_account,
                account::get_account_by_stellar_address
            ],
        )
        .mount(
            "/v1/payment",
            routes![
                payment::establish_trustline_for_non_native_asset,
                payment::send_native_payment,
                payment::send_non_native_payment
            ],
        )
}
