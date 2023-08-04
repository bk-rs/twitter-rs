// https://developer.twitter.com/en/support/twitter-api/error-troubleshooting

use serde::{Deserialize, Serialize};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Error {
    pub status: Option<u16>,
    pub r#type: String,
    pub title: String,
    pub detail: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = r#"
        {
            "client_id": "101010101",
            "required_enrollment": "Standard Basic",
            "registration_url": "https://developer.twitter.com/en/account",
            "title": "Client Forbidden",
            "detail": "This request must be made using an approved developer account that is enrolled in the requested endpoint. Learn more by visiting our documentation.",
            "reason": "client-not-enrolled",
            "type": "https://api.twitter.com/2/problems/client-forbidden"
        }
        "#;
        match serde_json::from_str::<Error>(json) {
            Ok(error) => {
                assert_eq!(error.title, "Client Forbidden");
            }
            Err(err) => panic!("{err}"),
        }
    }
}
