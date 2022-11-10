use url::{ParseError as UrlParseError, Url};

use crate::endpoints::{authenticate::AuthenticateEndpoint, authorize::AuthorizeEndpoint};

//
pub fn build_authorization_url(
    is_authenticate: bool,
    oauth_token: impl AsRef<str>,
    force_login: Option<bool>,
    screen_name: Option<&str>,
) -> Result<Url, UrlParseError> {
    if is_authenticate {
        let mut ep = AuthenticateEndpoint::new(oauth_token.as_ref());
        if let Some(force_login) = force_login {
            ep = ep.with_force_login(force_login);
        }
        if let Some(screen_name) = screen_name {
            ep = ep.with_screen_name(screen_name);
        }
        if let Ok(x) = ep.authorization_url() {
            return x.parse::<Url>();
        }
    } else {
        let mut ep = AuthorizeEndpoint::new(oauth_token.as_ref());
        if let Some(force_login) = force_login {
            ep = ep.with_force_login(force_login);
        }
        if let Some(screen_name) = screen_name {
            ep = ep.with_screen_name(screen_name);
        }
        if let Ok(x) = ep.authorization_url() {
            return x.parse::<Url>();
        }
    }

    build_authorization_url_inner(is_authenticate, oauth_token, force_login, screen_name)
}

fn build_authorization_url_inner(
    is_authenticate: bool,
    oauth_token: impl AsRef<str>,
    force_login: Option<bool>,
    screen_name: Option<&str>,
) -> Result<Url, UrlParseError> {
    let mut url = if is_authenticate {
        crate::endpoints::authenticate::URL.parse::<Url>()?
    } else {
        crate::endpoints::authorize::URL.parse::<Url>()?
    };
    url.query_pairs_mut()
        .append_pair("oauth_token", oauth_token.as_ref());
    if let Some(force_login) = force_login {
        url.query_pairs_mut()
            .append_pair("force_login", force_login.to_string().as_ref());
    }
    if let Some(screen_name) = screen_name {
        url.query_pairs_mut()
            .append_pair("screen_name", screen_name);
    }
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_authorization_url() {
        //
        assert_eq!(
            build_authorization_url(false, "Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hxxxxxx", None, None)
                .unwrap()
                .as_str(),
            "https://api.twitter.com/oauth/authorize?oauth_token=Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hxxxxxx"
        );
        assert_eq!(
            build_authorization_url_inner(false, "Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hxxxxxx", None, None)
                .unwrap()
                .as_str(),
            "https://api.twitter.com/oauth/authorize?oauth_token=Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hxxxxxx"
        );

        //
        assert_eq!(
            build_authorization_url(false, "Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hxxxxxx", Some(true), Some("xxx"))
                .unwrap()
                .as_str(),
            "https://api.twitter.com/oauth/authorize?oauth_token=Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hxxxxxx&force_login=true&screen_name=xxx"
        );
        assert_eq!(
            build_authorization_url_inner(false, "Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hxxxxxx", Some(true), Some("xxx"))
                .unwrap()
                .as_str(),
            "https://api.twitter.com/oauth/authorize?oauth_token=Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hxxxxxx&force_login=true&screen_name=xxx"
        );

        //
        assert_eq!(
            build_authorization_url(true, "Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hxxxxxx", None, None)
                .unwrap()
                .as_str(),
            "https://api.twitter.com/oauth/authenticate?oauth_token=Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hxxxxxx"
        );
        assert_eq!(
            build_authorization_url_inner(true, "Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hxxxxxx", None, None)
                .unwrap()
                .as_str(),
            "https://api.twitter.com/oauth/authenticate?oauth_token=Z6eEdO8MOmk394WozF5oKyuAv855l4Mlqo7hxxxxxx"
        );
    }
}
