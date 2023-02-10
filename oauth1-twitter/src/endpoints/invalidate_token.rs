//! https://developer.twitter.com/en/docs/authentication/api-reference/invalidate_access_token

use http_api_client_endpoint::{http::StatusCode, Body, Endpoint, Request, Response};
use serde::{Deserialize, Serialize};

use super::common::{EndpointError, EndpointRet};
use crate::objects::{
    authentication_access_token::AuthenticationAccessToken, consumer_key::ConsumerKey,
};

pub const URL: &str = "https://api.twitter.com/1.1/oauth/invalidate_token";

//
#[derive(Debug, Clone)]
pub struct InvalidateTokenEndpoint {
    pub consumer_key: ConsumerKey,
    pub authentication_access_token: AuthenticationAccessToken,
}
impl InvalidateTokenEndpoint {
    pub fn new(
        consumer_key: ConsumerKey,
        authentication_access_token: AuthenticationAccessToken,
    ) -> Self {
        Self {
            consumer_key,
            authentication_access_token,
        }
    }
}

impl Endpoint for InvalidateTokenEndpoint {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<InvalidateTokenResponseBody>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let request_tmp = reqwest_oauth1::Client::new()
            .post(URL)
            .sign(
                self.consumer_key
                    .secrets_with_access_token(&self.authentication_access_token),
            )
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
                serde_json::from_slice::<InvalidateTokenResponseBody>(response.body())
                    .map_err(EndpointError::DeResponseBodyOkJsonFailed)?,
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
pub struct InvalidateTokenResponseBody {
    pub access_token: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use http_api_client_endpoint::http::Method;

    #[test]
    fn test_render_request() {
        //
        let req = InvalidateTokenEndpoint::new(
            ConsumerKey::new("foo", "bar"),
            AuthenticationAccessToken::new("aaa", "xxx"),
        )
        .render_request()
        .unwrap();
        assert_eq!(req.method(), Method::POST);
        assert_eq!(
            req.uri(),
            "https://api.twitter.com/1.1/oauth/invalidate_token"
        );
        let req_header_authorization =
            String::from_utf8_lossy(req.headers().get("Authorization").unwrap().as_bytes());
        assert!(req_header_authorization.starts_with(r#"OAuth oauth_consumer_key="foo""#));
    }

    #[test]
    fn test_parse_response() {
        //
        let body = include_str!("../../tests/response_body_files/invalidate_access_token.json");
        let res = Response::builder()
            .status(StatusCode::OK)
            .body(body.as_bytes().to_owned())
            .unwrap();
        let ret = InvalidateTokenEndpoint::new(
            ConsumerKey::new("foo", "bar"),
            AuthenticationAccessToken::new("aaa", "xxx"),
        )
        .parse_response(res)
        .unwrap();
        match &ret {
            EndpointRet::Ok(body) => {
                assert_eq!(body.access_token, "ACCESS_TOKEN");
            }
            EndpointRet::Other(_) => panic!("{ret:?}"),
        }

        //
        let body =
            include_str!("../../tests/response_body_files/invalidate_access_token__401.json");
        let res = Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(body.as_bytes().to_owned())
            .unwrap();
        let ret = InvalidateTokenEndpoint::new(
            ConsumerKey::new("foo", "bar"),
            AuthenticationAccessToken::new("aaa", "xxx"),
        )
        .parse_response(res)
        .unwrap();
        match &ret {
            EndpointRet::Ok(_) => {
                panic!("{ret:?}")
            }
            EndpointRet::Other((status_code, body)) => {
                assert_eq!(status_code, &StatusCode::UNAUTHORIZED);
                assert_eq!(
                    body.as_ref().unwrap().errors.first().map(|x| x.code),
                    Some(89)
                );
            }
        }
    }
}
