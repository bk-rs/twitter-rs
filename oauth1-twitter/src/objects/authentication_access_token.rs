//
#[derive(Debug, Clone)]
pub struct AuthenticationAccessToken {
    pub access_token: String,
    pub secret: String,
}

impl AuthenticationAccessToken {
    pub fn new(access_token: impl AsRef<str>, secret: impl AsRef<str>) -> Self {
        Self {
            access_token: access_token.as_ref().into(),
            secret: secret.as_ref().into(),
        }
    }
}
