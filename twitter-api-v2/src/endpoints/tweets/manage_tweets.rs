// https://developer.twitter.com/en/docs/twitter-api/tweets/manage-tweets/introduction

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

//
// TODO,

//
pub const URL_FOR_TWEETS_CREATE: &str = "https://api.twitter.com/2/tweets";

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TweetsCreateResponseBody {
    pub data: TweetsCreateResponseBodyData,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TweetsCreateResponseBodyData {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u64,
    pub text: Option<String>,
}
