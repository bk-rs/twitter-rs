/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p twitter-api-v1-demo --bin twitter_api_v1_demo_show_user -- 'YOUR_CONSUMER_KEY' 'YOUR_CONSUMER_SECRET' 'YOUR_ACCESS_TOKEN' 'YOUR_ACCESS_TOKEN_SECRET' 'ID_OR_SCREEN_NAME'
*/

use std::env;

use twitter_api_v1::{
    endpoints::{
        users::lookup::{show_user_by_id, show_user_by_screen_name},
        EndpointRet,
    },
    TokenSecrets,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let consumer_key = env::args()
        .nth(1)
        .ok_or_else(|| "arg consumer_key missing".to_string())?;
    let consumer_secret = env::args()
        .nth(2)
        .ok_or_else(|| "arg consumer_secret missing".to_string())?;
    let oauth_token = env::args()
        .nth(3)
        .ok_or_else(|| "arg oauth_token missing".to_string())?;
    let oauth_token_secret = env::args()
        .nth(4)
        .ok_or_else(|| "arg oauth_token_secret missing".to_string())?;

    let id_or_screen_name = env::args()
        .nth(5)
        .ok_or_else(|| "arg id_or_screen_name missing".to_string())?;

    //
    let token_secrets = TokenSecrets::new(
        consumer_key,
        consumer_secret,
        oauth_token,
        oauth_token_secret,
    );

    if let Ok(id) = id_or_screen_name.parse::<u64>() {
        let ret = show_user_by_id(&token_secrets, reqwest::Client::new(), id, None).await?;
        match ret {
            EndpointRet::Ok(ok_json) => {
                println!("show_user_by_id:{ok_json:?}");
            }
            x => panic!("{x:?}"),
        };
    } else {
        let ret = show_user_by_screen_name(
            &token_secrets,
            reqwest::Client::new(),
            id_or_screen_name,
            None,
        )
        .await?;
        match ret {
            EndpointRet::Ok(ok_json) => {
                println!("show_user_by_screen_name:{ok_json:?}");
            }
            x => panic!("{x:?}"),
        };
    }

    Ok(())
}
