use serde::{Deserialize, Serialize};
use twitter_api_v2::objects::ResponseBodyErrJson as V2ResponseBodyErrJson;

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Error {
    pub errors: Vec<ErrorErrorsItem>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ErrorErrorsItem {
    pub code: i64,
    pub message: String,
}

//
impl From<V2ResponseBodyErrJson> for Error {
    fn from(value: V2ResponseBodyErrJson) -> Self {
        Self {
            errors: vec![ErrorErrorsItem {
                code: 0,
                message: value.detail,
            }],
        }
    }
}
