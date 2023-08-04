use reqwest::{Client, StatusCode};
use reqwest_oauth1::OAuthClientProvider as _;
use twitter_api_v2::{
    endpoints::users::lookup::{
        url_for_user_by_id, url_for_user_by_username, SingleUserResponseBody,
    },
    objects::ResponseBodyErrJson as V2ResponseBodyErrJson,
};

use crate::{
    endpoints::common::{EndpointError, EndpointRet},
    objects::User,
    secrets::TokenSecrets,
};

//
//
//
pub async fn show_user_by_id(
    secrets: &TokenSecrets,
    client: Client,
    user_id: u64,
    _include_entities: Option<bool>,
) -> Result<EndpointRet<User>, EndpointError> {
    //
    let url = url_for_user_by_id(user_id);

    //
    let response = client
        .oauth1(secrets.secrets())
        .get(url)
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
        StatusCode::OK => {
            let response_body = serde_json::from_slice::<SingleUserResponseBody>(response_body)
                .map_err(EndpointError::DeV2ResponseBodyOkJsonFailed)?;
            let user = User::try_from(response_body.data)
                .map_err(EndpointError::ConvertV2ResponseBodyOkJsonFailed)?;
            Ok(EndpointRet::Ok(user))
        }
        status => match serde_json::from_slice::<V2ResponseBodyErrJson>(response_body) {
            Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json.into())))),
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
    _include_entities: Option<bool>,
) -> Result<EndpointRet<User>, EndpointError> {
    //
    let url = url_for_user_by_username(screen_name);

    //
    let response = client
        .oauth1(secrets.secrets())
        .get(url)
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
        StatusCode::OK => {
            let response_body = serde_json::from_slice::<SingleUserResponseBody>(response_body)
                .map_err(EndpointError::DeV2ResponseBodyOkJsonFailed)?;
            let user = User::try_from(response_body.data)
                .map_err(EndpointError::ConvertV2ResponseBodyOkJsonFailed)?;
            Ok(EndpointRet::Ok(user))
        }
        status => match serde_json::from_slice::<V2ResponseBodyErrJson>(response_body) {
            Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json.into())))),
            Err(_) => Ok(EndpointRet::Other((status, Err(response_body.to_owned())))),
        },
    }
}
