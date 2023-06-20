// https://developer.twitter.com/en/docs/twitter-api/data-dictionary/object-model/user

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_option_number_from_string;
use serde_json::{Map, Value};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub id: Option<u64>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub entities: Option<Map<String, Value>>,
    pub location: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub pinned_tweet_id: Option<u64>,
    pub profile_image_url: Option<String>,
    pub protected: Option<bool>,
    pub public_metrics: Option<Map<String, Value>>,
    pub url: Option<String>,
    pub verified: Option<bool>,
    pub withheld: Option<Value>,
}

impl User {
    pub fn screen_name(&self) -> Option<&str> {
        self.username.as_deref()
    }
}
