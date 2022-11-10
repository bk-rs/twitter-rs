/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p oauth1-twitter-demo --bin oauth1_twitter_three_legged_oauth_flow -- 'YOUR_CONSUMER_KEY' 'YOUR_CONSUMER_SECRET' 'YOUR_CALLBACK_URL'
*/

// https://developer.twitter.com/en/docs/authentication/oauth-1-0a/obtaining-user-access-tokens

use std::{env, error};

use http_api_isahc_client::{Client as _, IsahcClient};
use log::info;
use oauth1_twitter::{
    endpoints::{
        AccessTokenEndpoint, AuthenticateEndpoint, EndpointRet, InvalidateTokenEndpoint,
        RequestTokenEndpoint,
    },
    objects::{CallbackUrlQuery, ConsumerKey},
};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    let consumer_key = env::args()
        .nth(1)
        .ok_or_else(|| "arg consumer_key missing".to_string())?;
    let consumer_secret = env::args()
        .nth(2)
        .ok_or_else(|| "arg consumer_secret missing".to_string())?;
    let callback_url = env::args()
        .nth(3)
        .ok_or_else(|| "arg callback_url missing".to_string())?;

    let consumer_key = ConsumerKey::new(consumer_key, consumer_secret);

    //
    let client = IsahcClient::new()?;

    //
    // Step 1
    //
    let request_token_ep = RequestTokenEndpoint::new(consumer_key.to_owned(), callback_url);
    let ret = client.respond_endpoint(&request_token_ep).await?;
    let request_token_res_body = match &ret {
        EndpointRet::Ok(body) => body,
        EndpointRet::Other((status_code, body)) => {
            return Err(format!(
                "request_token_ep status_code:{} body:{:?}",
                status_code, body
            )
            .into());
        }
    };
    info!("{:?}", request_token_res_body);
    assert!(request_token_res_body.oauth_callback_confirmed);

    //
    // Step 2
    //
    let authenticate_ep = AuthenticateEndpoint::new(&request_token_res_body.oauth_token);
    let authorization_url = authenticate_ep.authorization_url()?;

    println!("please open {}", authorization_url);

    println!("input callback_url: ");
    let mut callback_url = String::new();
    std::io::stdin().read_line(&mut callback_url)?;
    let callback_url = callback_url.trim();

    //
    let callback_url = Url::parse(callback_url)?;
    let callback_url_query = callback_url.query().unwrap_or_default();
    let callback_url_query: CallbackUrlQuery = callback_url_query.parse()?;
    assert_eq!(
        callback_url_query.oauth_token,
        request_token_res_body.oauth_token
    );

    //
    // Step 3
    //
    let access_token_ep = AccessTokenEndpoint::new(
        consumer_key.to_owned(),
        request_token_res_body.authentication_request_token(),
        &callback_url_query.oauth_verifier,
    );
    let ret = client.respond_endpoint(&access_token_ep).await?;
    let access_token_res_body = match &ret {
        EndpointRet::Ok(body) => body,
        EndpointRet::Other((status_code, body)) => {
            return Err(format!(
                "access_token_ep status_code:{} body:{:?}",
                status_code, body
            )
            .into());
        }
    };
    info!("{:?}", access_token_res_body);

    //
    //
    //
    let invalidate_token_ep = InvalidateTokenEndpoint::new(
        consumer_key,
        access_token_res_body.authentication_access_token(),
    );
    let ret = client.respond_endpoint(&invalidate_token_ep).await?;
    let invalidate_token_res_body = match &ret {
        EndpointRet::Ok(body) => body,
        EndpointRet::Other((status_code, body)) => {
            return Err(format!(
                "invalidate_token_ep status_code:{} body:{:?}",
                status_code, body
            )
            .into());
        }
    };
    info!("{:?}", invalidate_token_res_body);
    assert_eq!(
        invalidate_token_res_body.access_token,
        access_token_res_body.oauth_token
    );

    Ok(())
}
