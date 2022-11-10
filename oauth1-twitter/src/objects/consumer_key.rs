use reqwest_oauth1::Secrets;

use crate::objects::{
    authentication_access_token::AuthenticationAccessToken,
    authentication_request_token::AuthenticationRequestToken,
};

//
#[derive(Debug, Clone)]
pub struct ConsumerKey {
    pub key: String,
    pub secret: String,
}

impl ConsumerKey {
    pub fn new(key: impl AsRef<str>, secret: impl AsRef<str>) -> Self {
        Self {
            key: key.as_ref().into(),
            secret: secret.as_ref().into(),
        }
    }

    pub fn secrets(&self) -> Secrets {
        Secrets::new(&self.key, &self.secret)
    }

    pub fn secrets_with_request_token(
        &self,
        authentication_request_token: &AuthenticationRequestToken,
    ) -> Secrets {
        Secrets::new(&self.key, &self.secret).token(
            authentication_request_token.request_token.to_owned(),
            authentication_request_token.secret.to_owned(),
        )
    }

    pub fn secrets_with_access_token(
        &self,
        authentication_access_token: &AuthenticationAccessToken,
    ) -> Secrets {
        Secrets::new(&self.key, &self.secret).token(
            authentication_access_token.access_token.to_owned(),
            authentication_access_token.secret.to_owned(),
        )
    }
}
