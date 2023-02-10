//! https://developer.twitter.com/en/docs/authentication/api-reference/access_token

use http_api_client_endpoint::{http::StatusCode, Body, Endpoint, Request, Response};
use serde::{Deserialize, Serialize};

use super::common::{EndpointError, EndpointRet};
use crate::objects::{
    authentication_access_token::AuthenticationAccessToken,
    authentication_request_token::AuthenticationRequestToken, consumer_key::ConsumerKey,
};

pub const URL: &str = "https://api.twitter.com/oauth/access_token";

//
#[derive(Debug, Clone)]
pub struct AccessTokenEndpoint {
    pub consumer_key: ConsumerKey,
    pub authentication_request_token: AuthenticationRequestToken,
    pub oauth_verifier: String,
}
impl AccessTokenEndpoint {
    pub fn new(
        consumer_key: ConsumerKey,
        authentication_request_token: AuthenticationRequestToken,
        oauth_verifier: impl AsRef<str>,
    ) -> Self {
        Self {
            consumer_key,
            authentication_request_token,
            oauth_verifier: oauth_verifier.as_ref().into(),
        }
    }
}

impl Endpoint for AccessTokenEndpoint {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<AccessTokenResponseBody>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let query = AccessTokenRequestQuery {
            oauth_verifier: self.oauth_verifier.to_owned(),
        };

        let request_tmp = reqwest_oauth1::Client::new()
            .post(URL)
            .sign(
                self.consumer_key
                    .secrets_with_request_token(&self.authentication_request_token),
            )
            .query(&query)
            .generate_signature()
            .map_err(EndpointError::MakeReqwestRequestBuilderFailed)?
            .build()
            .map_err(EndpointError::MakeReqwestRequestFailed)?;

        let mut request = Request::builder()
            .method(request_tmp.method())
            .uri(request_tmp.url().as_str())
            .body(vec![])
            .map_err(EndpointError::MakeRequestFailed)?;

        let headers = request.headers_mut();
        *headers = request_tmp.headers().to_owned();

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let status = response.status();

        match status {
            StatusCode::OK => Ok(EndpointRet::Ok(
                serde_urlencoded::from_bytes::<AccessTokenResponseBody>(response.body())
                    .map_err(EndpointError::DeResponseBodyOkFailed)?,
            )),
            status => match serde_json::from_slice(response.body()) {
                Ok(fail_json) => Ok(EndpointRet::Other((status, Ok(fail_json)))),
                Err(_) => Ok(EndpointRet::Other((
                    status,
                    Err(response.body().to_owned()),
                ))),
            },
        }
    }
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AccessTokenRequestQuery {
    pub oauth_verifier: String,
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AccessTokenResponseBody {
    pub oauth_token: String,
    pub oauth_token_secret: String,
    pub user_id: u64,
    pub screen_name: String,
}

impl AccessTokenResponseBody {
    pub fn authentication_access_token(&self) -> AuthenticationAccessToken {
        AuthenticationAccessToken::new(&self.oauth_token, &self.oauth_token_secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use http_api_client_endpoint::http::Method;

    #[test]
    fn test_render_request() {
        //
        let req = AccessTokenEndpoint::new(
            ConsumerKey::new("foo", "bar"),
            AuthenticationRequestToken::new("aaa", "xxx"),
            "bbb",
        )
        .render_request()
        .unwrap();
        assert_eq!(req.method(), Method::POST);
        assert_eq!(req.uri(), "https://api.twitter.com/oauth/access_token");
        let req_header_authorization =
            String::from_utf8_lossy(req.headers().get("Authorization").unwrap().as_bytes());
        assert!(req_header_authorization.starts_with(r#"OAuth oauth_consumer_key="foo""#));
        assert!(req_header_authorization.contains(r#"oauth_verifier="bbb""#));
    }

    #[test]
    fn test_parse_response() {
        //
        let body = include_str!("../../tests/response_body_files/access_token.txt");
        let res = Response::builder()
            .status(StatusCode::OK)
            .body(body.as_bytes().to_owned())
            .unwrap();
        let ret = AccessTokenEndpoint::new(
            ConsumerKey::new("foo", "bar"),
            AuthenticationRequestToken::new("aaa", "xxx"),
            "bbb",
        )
        .parse_response(res)
        .unwrap();
        match &ret {
            EndpointRet::Ok(body) => {
                assert_eq!(
                    body.oauth_token,
                    "62532xx-eWudHldSbIaelX7swmsiHImEL4KinwaGloxxxxxx"
                );
                assert_eq!(
                    body.oauth_token_secret,
                    "2EEfA6BG5ly3sR3XjE0IBSnlQu4ZrUzPiYxxxxxx"
                );
                assert_eq!(body.user_id, 6253282);
                assert_eq!(body.screen_name, "twitterapi");
            }
            EndpointRet::Other(_) => panic!("{ret:?}"),
        }
    }
}
