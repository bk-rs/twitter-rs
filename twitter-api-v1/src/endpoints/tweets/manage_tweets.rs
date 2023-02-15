use reqwest::{Client, StatusCode};
use reqwest_oauth1::OAuthClientProvider as _;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{
    endpoints::common::{EndpointError, EndpointRet},
    objects::ResponseBodyErrJson,
    secrets::TokenSecrets,
};

//
pub const CREATE_TWEET_URL: &str = "https://api.twitter.com/1.1/statuses/update.json";

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
        form.insert("status".into(), status.into());
    }
    if let Some(media_ids) = media_ids {
        form.insert(
            "media_ids".into(),
            media_ids
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
                .into(),
        );
    }
    if let Some(mut other_parameters) = other_parameters {
        form.append(&mut other_parameters);
    }

    //
    let response = client
        .oauth1(secrets.secrets())
        .post(CREATE_TWEET_URL)
        .form(&form)
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
        StatusCode::OK => Ok(EndpointRet::Ok(
            serde_json::from_slice(response_body)
                .map_err(EndpointError::DeResponseBodyOkJsonFailed)?,
        )),
        status => match serde_json::from_slice::<ResponseBodyErrJson>(response_body) {
            Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json)))),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_create_tweet_response_body_ok_json() {
        match serde_json::from_str::<CreateTweetResponseBodyOkJson>(include_str!(
            "../../../tests/response_body_json_files/tweets__create_tweet__ok.json"
        )) {
            Ok(ok_json) => {
                assert_eq!(ok_json.id, 1050118621198921700);
            }
            Err(err) => panic!("{err}"),
        }
    }
}
