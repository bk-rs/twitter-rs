use serde::{Deserialize, Serialize};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseBodyFail {
    pub errors: Vec<ResponseBodyFailError>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseBodyFailError {
    pub code: i64,
    pub message: String,
}
