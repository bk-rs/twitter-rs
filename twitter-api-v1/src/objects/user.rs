use serde::{Deserialize, Serialize};
use twitter_api_v2::objects::User as V2User;

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

//
impl TryFrom<V2User> for User {
    type Error = String;
    fn try_from(value: V2User) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.ok_or("id missing")?,
            id_str: value.id.ok_or("id missing")?.to_string(),
            screen_name: value.username.ok_or("username missing")?,
            profile_banner_url: None,
            profile_image_url_https: value.profile_image_url.ok_or("profile_image_url missing")?,
            default_profile_image: false,
        })
    }
}
