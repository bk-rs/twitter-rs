//
#[derive(Debug, Clone)]
pub struct AuthenticationRequestToken {
    pub request_token: String,
    pub secret: String,
}

impl AuthenticationRequestToken {
    pub fn new(request_token: impl AsRef<str>, secret: impl AsRef<str>) -> Self {
        Self {
            request_token: request_token.as_ref().into(),
            secret: secret.as_ref().into(),
        }
    }
}
