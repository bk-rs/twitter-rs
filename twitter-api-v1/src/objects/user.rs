use serde::{Deserialize, Serialize};

//
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: u64,
    pub id_str: String,
    pub screen_name: String,
    pub profile_banner_url: Option<String>,
    pub profile_image_url_https: String,
    pub default_profile_image: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_user() {
        match serde_json::from_str::<User>(include_str!(
            "../../tests/response_body_json_files/user.json"
        )) {
            Ok(user) => {
                assert_eq!(user.id, 6253282);
            }
            Err(err) => panic!("{err}"),
        }
    }
}
