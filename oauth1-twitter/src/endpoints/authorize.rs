//! https://developer.twitter.com/en/docs/authentication/api-reference/authorize

use http_api_client_endpoint::{
    http::{Error as HttpError, Method},
    Body, Endpoint, Request, Response,
};
use serde::{Deserialize, Serialize};
use serde_qs::Error as SerdeQsError;
use url::{ParseError as UrlParseError, Url};

pub const URL: &str = "https://api.twitter.com/oauth/authorize";

//
#[derive(Debug, Clone)]
pub struct AuthorizeEndpoint {
    pub oauth_token: String,
    pub force_login: Option<bool>,
    pub screen_name: Option<String>,
}
impl AuthorizeEndpoint {
    pub fn new(oauth_token: impl AsRef<str>) -> Self {
        Self {
            oauth_token: oauth_token.as_ref().into(),
            force_login: None,
            screen_name: None,
        }
    }

    pub fn with_force_login(mut self, force_login: bool) -> Self {
        self.force_login = Some(force_login);
        self
    }

    pub fn with_screen_name(mut self, screen_name: impl AsRef<str>) -> Self {
        self.screen_name = Some(screen_name.as_ref().into());
        self
    }

    pub fn authorization_url(&self) -> Result<String, AuthorizeEndpointError> {
        let request = self.render_request()?;
        Ok(request.uri().to_string())
    }
}

impl Endpoint for AuthorizeEndpoint {
    type RenderRequestError = AuthorizeEndpointError;

    type ParseResponseOutput = ();
    type ParseResponseError = AuthorizeEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let mut url = Url::parse(URL).map_err(AuthorizeEndpointError::MakeRequestUrlFailed)?;

        let query = AuthorizeRequestQuery {
            oauth_token: self.oauth_token.to_owned(),
            force_login: self.force_login,
            screen_name: self.screen_name.to_owned(),
        };

        let query = serde_qs::to_string(&query)
            .map_err(AuthorizeEndpointError::SerRequestUrlQueryFailed)?;

        url.set_query(Some(query.as_str()));

        let request = Request::builder()
            .method(Method::GET)
            .uri(url.as_str())
            .body(vec![])
            .map_err(AuthorizeEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        _response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        unreachable!()
    }
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthorizeRequestQuery {
    pub oauth_token: String,
    pub force_login: Option<bool>,
    pub screen_name: Option<String>,
}

//
//
//
#[derive(Debug)]
pub enum AuthorizeEndpointError {
    MakeRequestUrlFailed(UrlParseError),
    SerRequestUrlQueryFailed(SerdeQsError),
    MakeRequestFailed(HttpError),
}

impl core::fmt::Display for AuthorizeEndpointError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AuthorizeEndpointError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_request() {
        //
        let req = AuthorizeEndpoint::new("Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hxxxxxx")
            .render_request()
            .unwrap();
        assert_eq!(req.method(), Method::GET);
        assert_eq!(req.uri(), "https://api.twitter.com/oauth/authorize?oauth_token=Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hxxxxxx");

        //
        let req = AuthorizeEndpoint::new("Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hxxxxxx")
            .with_force_login(true)
            .with_screen_name("xxx")
            .render_request()
            .unwrap();
        assert_eq!(req.method(), Method::GET);
        assert_eq!(req.uri(), "https://api.twitter.com/oauth/authorize?oauth_token=Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hxxxxxx&force_login=true&screen_name=xxx");
    }
}
