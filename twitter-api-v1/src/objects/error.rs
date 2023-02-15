use serde::{Deserialize, Serialize};

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
