use reqwest::{Client, StatusCode};
use reqwest_oauth1::OAuthClientProvider as _;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use twitter_api_v2::{
    endpoints::tweets::manage_tweets::{
        TweetsCreateResponseBody as V2TweetsCreateResponseBody,
        URL_FOR_TWEETS_CREATE as V2_URL_FOR_TWEETS_CREATE,
    },
    objects::ResponseBodyErrJson as V2ResponseBodyErrJson,
};

use crate::{
    endpoints::common::{EndpointError, EndpointRet},
    secrets::TokenSecrets,
};

//
//
//
pub async fn create_tweet(
    secrets: &TokenSecrets,
    client: Client,
    status: Option<&str>,
    media_ids: Option<Vec<u64>>,
    other_parameters: Option<Map<String, Value>>,
) -> Result<EndpointRet<CreateTweetResponseBodyOkJson>, EndpointError> {
    if status.is_none() && media_ids.is_none() {
        return Err(EndpointError::ValidateFailed(
            "status is required if media_ids is not present.".into(),
        ));
    }

    //
    let mut form = Map::new();
    if let Some(status) = status {
        form.insert("text".into(), status.into());
    }
    if let Some(media_ids) = media_ids {
        let mut media = Map::new();
        media.insert(
            "media_ids".into(),
            media_ids
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .into(),
        );

        form.insert("media".into(), media.into());
    }
    if let Some(mut other_parameters) = other_parameters {
        form.append(&mut other_parameters);
    }

    //
    let response = client
        .oauth1(secrets.secrets())
        .post(V2_URL_FOR_TWEETS_CREATE)
        .json(&form)
        .send()
        .await
        .map_err(EndpointError::RespondFailed)?;

    //
    let response_status = response.status();
    let response_body = response
        .bytes()
        .await
        .map_err(EndpointError::ReadResponseBodyFailed)?;
    let response_body = response_body.as_ref();

    match response_status {
        StatusCode::CREATED => {
            let response_body = serde_json::from_slice::<V2TweetsCreateResponseBody>(response_body)
                .map_err(EndpointError::DeV2ResponseBodyOkJsonFailed)?;
            Ok(EndpointRet::Ok(CreateTweetResponseBodyOkJson::from(
                response_body,
            )))
        }
        status => match serde_json::from_slice::<V2ResponseBodyErrJson>(response_body) {
            Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json.into())))),
            Err(_) => Ok(EndpointRet::Other((status, Err(response_body.to_owned())))),
        },
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateTweetResponseBodyOkJson {
    pub id: u64,
    #[serde(rename = "id_str")]
    pub id_string: String,
}

//
impl From<V2TweetsCreateResponseBody> for CreateTweetResponseBodyOkJson {
    fn from(value: V2TweetsCreateResponseBody) -> Self {
        Self {
            id: value.data.id,
            id_string: value.data.id.to_string(),
        }
    }
}
