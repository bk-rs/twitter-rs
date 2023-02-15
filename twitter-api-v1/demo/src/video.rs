/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p twitter-api-v1-demo --bin twitter_api_v1_demo_video -- 'YOUR_CONSUMER_KEY' 'YOUR_CONSUMER_SECRET' 'YOUR_ACCESS_TOKEN' 'YOUR_ACCESS_TOKEN_SECRET' '/path/x.mp4' 'Hello'
*/

use std::env;

use twitter_api_v1::{
    endpoints::{
        media::upload_media::{
            get_upload_status, upload_append_all_from_file, upload_finalize, upload_init,
        },
        tweets::manage_tweets::create_tweet,
        EndpointRet,
    },
    objects::{MediaCategory, MediaProcessingInfoState},
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

    let twitter_api_v1::tokio_fs_util::Info {
        file_size,
        file_name: _,
    } = twitter_api_v1::tokio_fs_util::info(&file_path).await?;

    //
    // upload INIT
    //
    let ret = upload_init(
        &token_secrets,
        reqwest::Client::builder()
            .connection_verbose(true)
            .build()?,
        file_size as usize,
        "video/mp4",
        MediaCategory::TweetVideo,
    )
    .await?;
    let media_id = match ret {
        EndpointRet::Ok(ok_json) => {
            println!("upload_init:{ok_json:?}");
            ok_json.media_id
        }
        x => panic!("{x:?}"),
    };

    //
    // upload APPEND
    //
    match upload_append_all_from_file(&token_secrets, reqwest::Client::new(), media_id, &file_path)
        .await
    {
        Ok(Ok(_)) => {}
        Ok(Err(ret)) => panic!("{ret:?}"),
        Err(err) => panic!("{err:?}"),
    }

    //
    // upload FINALIZE
    //
    let ret = upload_finalize(
        &token_secrets,
        reqwest::Client::builder()
            .connection_verbose(true)
            .build()?,
        media_id,
    )
    .await?;
    let processing_info = match ret {
        EndpointRet::Ok(ok_json) => {
            println!("upload_finalize:{ok_json:?}");
            ok_json.processing_info
        }
        x => panic!("{x:?}"),
    };

    if processing_info.is_some() {
        //
        // upload STATUS
        //
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            let ret = get_upload_status(
                &token_secrets,
                reqwest::Client::builder()
                    .connection_verbose(true)
                    .build()?,
                media_id,
            )
            .await?;
            let processing_info = match ret {
                EndpointRet::Ok(ok_json) => {
                    println!("get_upload_status:{ok_json:?}");
                    ok_json.processing_info
                }
                x => panic!("{x:?}"),
            };

            if let Some(processing_info) = processing_info {
                match processing_info.state {
                    MediaProcessingInfoState::Succeeded => break,
                    MediaProcessingInfoState::Failed => panic!("{processing_info:?}"),
                    _ => {}
                }
            }
        }
    }

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
