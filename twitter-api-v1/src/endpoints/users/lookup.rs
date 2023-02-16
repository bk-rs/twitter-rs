use reqwest::{Client, StatusCode};
use reqwest_oauth1::OAuthClientProvider as _;
use serde_json::Map;

use crate::{
    endpoints::common::{EndpointError, EndpointRet},
    objects::{ResponseBodyErrJson, User},
    secrets::TokenSecrets,
};

//
pub const SHOW_USER_URL: &str = "https://api.twitter.com/1.1/users/show.json";

//
//
//
pub async fn show_user_by_id(
    secrets: &TokenSecrets,
    client: Client,
    user_id: u64,
    include_entities: Option<bool>,
) -> Result<EndpointRet<User>, EndpointError> {
    //
    let mut query = Map::new();
    query.insert("user_id".into(), user_id.into());
    if let Some(include_entities) = include_entities {
        query.insert("include_entities".into(), include_entities.into());
    }

    //
    let response = client
        .oauth1(secrets.secrets())
        .get(SHOW_USER_URL)
        .query(&query)
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

//
//
//
pub async fn show_user_by_screen_name(
    secrets: &TokenSecrets,
    client: Client,
    screen_name: impl AsRef<str>,
    include_entities: Option<bool>,
) -> Result<EndpointRet<User>, EndpointError> {
    //
    let mut query = Map::new();
    query.insert("screen_name".into(), screen_name.as_ref().into());
    if let Some(include_entities) = include_entities {
        query.insert("include_entities".into(), include_entities.into());
    }

    //
    let response = client
        .oauth1(secrets.secrets())
        .get(SHOW_USER_URL)
        .query(&query)
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
