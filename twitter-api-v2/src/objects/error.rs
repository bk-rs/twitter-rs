// https://developer.twitter.com/en/support/twitter-api/error-troubleshooting

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Error {
    pub r#type: String,
    pub title: String,
    pub detail: String,
    #[serde(flatten)]
    pub _others: Map<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_error() {
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

        //
        match serde_json::from_str::<Error>(include_str!(
            "../../tests/response_body_json_files/tweets__manage_tweets__create__err.json"
        )) {
            Ok(error) => {
                assert_eq!(error.title, "Invalid Request");
            }
            Err(err) => panic!("{err}"),
        }
    }
}
