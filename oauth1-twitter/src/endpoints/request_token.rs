//! https://developer.twitter.com/en/docs/authentication/api-reference/request_token

use http_api_client_endpoint::{http::StatusCode, Body, Endpoint, Request, Response};
use serde::{Deserialize, Serialize};

use super::common::{EndpointError, EndpointRet};
use crate::objects::{
    authentication_request_token::AuthenticationRequestToken, consumer_key::ConsumerKey,
};

pub const URL: &str = "https://api.twitter.com/oauth/request_token";

//
#[derive(Debug, Clone)]
pub struct RequestTokenEndpoint {
    pub consumer_key: ConsumerKey,
    pub oauth_callback: String,
    pub x_auth_access_type: Option<String>,
}
impl RequestTokenEndpoint {
    pub fn new(consumer_key: ConsumerKey, oauth_callback: impl AsRef<str>) -> Self {
        Self {
            consumer_key,
            oauth_callback: oauth_callback.as_ref().into(),
            x_auth_access_type: None,
        }
    }

    pub fn with_x_auth_access_type(mut self, x_auth_access_type: impl AsRef<str>) -> Self {
        self.x_auth_access_type = Some(x_auth_access_type.as_ref().into());
        self
    }
}

impl Endpoint for RequestTokenEndpoint {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<RequestTokenResponseBody>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let query = RequestTokenRequestQuery {
            oauth_callback: self.oauth_callback.to_owned(),
            x_auth_access_type: self.x_auth_access_type.to_owned(),
        };

        let request_tmp = reqwest_oauth1::Client::new()
            .post(URL)
            .sign(self.consumer_key.secrets())
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
                serde_urlencoded::from_bytes::<RequestTokenResponseBody>(response.body())
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
pub struct RequestTokenRequestQuery {
    pub oauth_callback: String,
    pub x_auth_access_type: Option<String>,
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RequestTokenResponseBody {
    pub oauth_token: String,
    pub oauth_token_secret: String,
    pub oauth_callback_confirmed: bool,
}

impl RequestTokenResponseBody {
    pub fn authentication_request_token(&self) -> AuthenticationRequestToken {
        AuthenticationRequestToken::new(&self.oauth_token, &self.oauth_token_secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use http_api_client_endpoint::http::Method;

    #[test]
    fn test_render_request() {
        //
        let req = RequestTokenEndpoint::new(
            ConsumerKey::new("foo", "bar"),
            "http://examplecallbackurl.local/auth.php",
        )
        .render_request()
        .unwrap();
        assert_eq!(req.method(), Method::POST);
        assert_eq!(req.uri(), "https://api.twitter.com/oauth/request_token");
        let req_header_authorization =
            String::from_utf8_lossy(req.headers().get("Authorization").unwrap().as_bytes());
        assert!(
            req_header_authorization.starts_with(r#"OAuth oauth_callback="http%3A%2F%2Fexamplecallbackurl.local%2Fauth.php",oauth_consumer_key="foo""#)
        );

        //
        let req = RequestTokenEndpoint::new(
            ConsumerKey::new("foo", "bar"),
            "http://examplecallbackurl.local/auth.php",
        )
        .with_x_auth_access_type("write")
        .render_request()
        .unwrap();
        assert_eq!(req.method(), Method::POST);
        assert_eq!(
            req.uri(),
            "https://api.twitter.com/oauth/request_token?x_auth_access_type=write"
        );
        assert!(
            String::from_utf8_lossy(req.headers().get("Authorization").unwrap().as_bytes())
                .starts_with(r#"OAuth oauth_callback="http%3A%2F%2Fexamplecallbackurl.local%2Fauth.php",oauth_consumer_key="foo""#)
        );
    }

    #[test]
    fn test_parse_response() {
        //
        let body = include_str!("../../tests/response_body_files/request_token.txt");
        let res = Response::builder()
            .status(StatusCode::OK)
            .body(body.as_bytes().to_owned())
            .unwrap();
        let ret = RequestTokenEndpoint::new(
            ConsumerKey::new("foo", "bar"),
            "http://examplecallbackurl.local/auth.php",
        )
        .parse_response(res)
        .unwrap();
        match &ret {
            EndpointRet::Ok(body) => {
                assert_eq!(body.oauth_token, "zlgW3QAAAAAA2_NZAAABfxxxxxxk");
                assert_eq!(body.oauth_token_secret, "pBYEQzdbyMqIcyDzyn0X7LDxxxxxxxxx");
                assert!(body.oauth_callback_confirmed);
            }
            EndpointRet::Other(_) => panic!("{:?}", ret),
        }

        //
        let body = include_str!("../../tests/response_body_files/request_token__400.json");
        let res = Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(body.as_bytes().to_owned())
            .unwrap();
        let ret = RequestTokenEndpoint::new(
            ConsumerKey::new("foo", "bar"),
            "http://examplecallbackurl.local/auth.php",
        )
        .parse_response(res)
        .unwrap();
        match &ret {
            EndpointRet::Ok(_) => {
                panic!("{:?}", ret)
            }
            EndpointRet::Other((status_code, body)) => {
                assert_eq!(status_code, &StatusCode::BAD_REQUEST);
                assert_eq!(
                    body.as_ref().unwrap().errors.first().map(|x| x.code),
                    Some(215)
                );
            }
        }
    }
}
