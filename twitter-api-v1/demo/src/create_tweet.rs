/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p twitter-api-v1-demo --bin twitter_api_v1_demo_create_tweet -- 'YOUR_CONSUMER_KEY' 'YOUR_CONSUMER_SECRET' 'YOUR_ACCESS_TOKEN' 'YOUR_ACCESS_TOKEN_SECRET' 'status' 'media_ids'
*/

use std::env;

use twitter_api_v1::{
    endpoints::{tweets::manage_tweets::create_tweet, EndpointRet},
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

    let status = env::args()
        .nth(5)
        .ok_or_else(|| "arg status missing".to_string())?;
    let media_ids = env::args().nth(6).map(|x| {
        x.split(',')
            .flat_map(|y| y.parse::<u64>().ok())
            .collect::<Vec<_>>()
    });

    //
    let token_secrets = TokenSecrets::new(
        consumer_key,
        consumer_secret,
        oauth_token,
        oauth_token_secret,
    );

    let client = reqwest::Client::builder()
        .connection_verbose(env::var("RUST_LOG").map(|x| x.starts_with("trace")) == Ok(true))
        .danger_accept_invalid_certs(true)
        .build()?;

    let ret = create_tweet(&token_secrets, client, Some(&status), media_ids, None).await?;
    match ret {
        EndpointRet::Ok(ok_json) => {
            println!("create_tweet:{ok_json:?}");
        }
        x => panic!("{x:?}"),
    };

    Ok(())
}
