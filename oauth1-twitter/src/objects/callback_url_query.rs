use core::str::FromStr;

use serde::{Deserialize, Serialize};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CallbackUrlQuery {
    pub oauth_token: String,
    pub oauth_verifier: String,
}

impl FromStr for CallbackUrlQuery {
    type Err = serde_qs::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_qs::from_str(s)
    }
}
