/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p twitter-api-v1-demo --bin twitter_api_v1_demo_image -- 'YOUR_CONSUMER_KEY' 'YOUR_CONSUMER_SECRET' 'YOUR_ACCESS_TOKEN' 'YOUR_ACCESS_TOKEN_SECRET' '/path/x.jpg' 'Hello'
*/

use std::env;

use twitter_api_v1::{
    endpoints::{
        media::upload_media::upload_image_from_file, tweets::manage_tweets::create_tweet,
        EndpointRet,
    },
    objects::MediaCategory,
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

    let file_path = env::args()
        .nth(5)
        .ok_or_else(|| "arg file_path missing".to_string())?
        .parse()?;
    let tweet_text = env::args().nth(6);

    //
    let token_secrets = TokenSecrets::new(
        consumer_key,
        consumer_secret,
        oauth_token,
        oauth_token_secret,
    );

    //
    // upload
    //
    let ret = upload_image_from_file(
        &token_secrets,
        reqwest::Client::new(),
        MediaCategory::TweetImage,
        &file_path,
    )
    .await?;
    let media_id = match ret {
        EndpointRet::Ok(ok_json) => {
            println!("upload_image:{ok_json:?}");
            ok_json.media_id
        }
        x => panic!("{x:?}"),
    };

    //
    // tweets
    //
    let ret = create_tweet(
        &token_secrets,
        reqwest::Client::builder()
            .connection_verbose(true)
            .build()?,
        tweet_text.as_deref(),
        Some(vec![media_id]),
        None,
    )
    .await?;
    match ret {
        EndpointRet::Ok(ok_json) => {
            println!("create_tweet:{ok_json:?}");
        }
        x => panic!("{x:?}"),
    };

    Ok(())
}
