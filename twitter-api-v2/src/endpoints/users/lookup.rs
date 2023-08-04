// https://developer.twitter.com/en/docs/twitter-api/users/lookup/introduction

use serde::{Deserialize, Serialize};

use crate::objects::User;

//
// TODO,

//
pub fn url_for_user_by_id(id: u64) -> String {
    format!("https://api.twitter.com/2/users/{id}")
}

pub fn url_for_user_by_username(username: impl AsRef<str>) -> String {
    let username = username.as_ref();
    format!("https://api.twitter.com/2/users/by/username/{username}")
}

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SingleUserResponseBody {
    pub data: User,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MultipleUsersResponseBody {
    pub data: Vec<User>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_response_body() {
        match serde_json::from_str::<SingleUserResponseBody>(include_str!(
            "../../../tests/response_body_json_files/users_lookup__me__default_fields.json"
        )) {
            Ok(body) => {
                assert_eq!(body.data.id.unwrap(), 2244994945);
            }
            Err(err) => panic!("{err}"),
        }

        match serde_json::from_str::<SingleUserResponseBody>(include_str!(
            "../../../tests/response_body_json_files/users_lookup__me__optional_fields.json"
        )) {
            Ok(body) => {
                println!("{body:?}");
                assert_eq!(body.data.id.unwrap(), 2244994945);
            }
            Err(err) => panic!("{err}"),
        }
    }
}
