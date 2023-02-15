use reqwest_oauth1::Secrets;

//
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct TokenSecrets {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub oauth_token: String,
    pub oauth_token_secret: String,
}
impl TokenSecrets {
    pub fn new(
        consumer_key: impl AsRef<str>,
        consumer_secret: impl AsRef<str>,
        oauth_token: impl AsRef<str>,
        oauth_token_secret: impl AsRef<str>,
    ) -> Self {
        Self {
            consumer_key: consumer_key.as_ref().into(),
            consumer_secret: consumer_secret.as_ref().into(),
            oauth_token: oauth_token.as_ref().into(),
            oauth_token_secret: oauth_token_secret.as_ref().into(),
        }
    }

    pub fn secrets(&self) -> Secrets {
        Secrets::new(&self.consumer_key, &self.consumer_secret)
            .token(&self.oauth_token, &self.oauth_token_secret)
    }
}
