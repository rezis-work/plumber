//! httpOnly refresh cookie attributes for `POST /auth/login` (Step 6).

use cookie::time::Duration as CookieDuration;
use cookie::{Cookie, SameSite};

const DEFAULT_COOKIE_NAME: &str = "refresh_token";
const DEFAULT_COOKIE_PATH: &str = "/auth";
const DEFAULT_SAMESITE: &str = "lax";

#[derive(Debug, Clone)]
pub struct CookieConfig {
    pub refresh_cookie_name: String,
    pub refresh_cookie_path: String,
    same_site: SameSite,
    secure: bool,
}

impl CookieConfig {
    pub fn from_env() -> Self {
        let refresh_cookie_name = std::env::var("AUTH_REFRESH_COOKIE_NAME")
            .unwrap_or_else(|_| DEFAULT_COOKIE_NAME.to_string());
        let refresh_cookie_path = std::env::var("AUTH_REFRESH_COOKIE_PATH")
            .unwrap_or_else(|_| DEFAULT_COOKIE_PATH.to_string());
        let same_site = parse_same_site(
            &std::env::var("AUTH_REFRESH_COOKIE_SAMESITE").unwrap_or_else(|_| DEFAULT_SAMESITE.to_string()),
        );
        let secure = parse_bool_env("AUTH_REFRESH_COOKIE_SECURE")
            || std::env::var("APP_ENV").as_deref() == Ok("production");
        Self {
            refresh_cookie_name,
            refresh_cookie_path,
            same_site,
            secure,
        }
    }

    /// Parse the refresh JWT from a raw `Cookie` request header (e.g. `refresh_token=...; other=x`).
    pub fn refresh_from_cookie_header(&self, cookie_header: &str) -> Option<String> {
        for parsed in Cookie::split_parse(cookie_header) {
            let Ok(c) = parsed else {
                continue;
            };
            if c.name() == self.refresh_cookie_name && !c.value().is_empty() {
                return Some(c.value().to_string());
            }
        }
        None
    }

    /// Full `Set-Cookie` header value (name=value; attributes).
    pub fn refresh_set_cookie_string(
        &self,
        refresh_jwt: &str,
        max_age_secs: i64,
    ) -> Result<String, ()> {
        let max_age = max_age_secs.max(0);
        let cookie = Cookie::build((self.refresh_cookie_name.clone(), refresh_jwt))
            .path(self.refresh_cookie_path.clone())
            .http_only(true)
            .same_site(self.same_site)
            .secure(self.secure)
            .max_age(CookieDuration::seconds(max_age))
            .build();
        Ok(cookie.to_string())
    }

    /// `Set-Cookie` that clears the refresh cookie (Step 10 logout): same Path / SameSite / Secure / HttpOnly as login.
    pub fn refresh_clear_cookie_string(&self) -> Result<String, ()> {
        let cookie = Cookie::build((self.refresh_cookie_name.clone(), ""))
            .path(self.refresh_cookie_path.clone())
            .http_only(true)
            .same_site(self.same_site)
            .secure(self.secure)
            .max_age(CookieDuration::seconds(0))
            .build();
        Ok(cookie.to_string())
    }
}

fn parse_bool_env(key: &str) -> bool {
    matches!(
        std::env::var(key).as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE") | Ok("yes") | Ok("YES")
    )
}

fn parse_same_site(raw: &str) -> SameSite {
    match raw.trim().to_ascii_lowercase().as_str() {
        "strict" => SameSite::Strict,
        "none" => SameSite::None,
        _ => SameSite::Lax,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> CookieConfig {
        CookieConfig {
            refresh_cookie_name: "refresh_token".to_string(),
            refresh_cookie_path: "/auth".to_string(),
            same_site: SameSite::Lax,
            secure: true,
        }
    }

    #[test]
    fn refresh_clear_cookie_string_max_age_zero_and_matches_set_attrs() {
        let c = test_config();
        let clear = c.refresh_clear_cookie_string().expect("clear");
        let set = c
            .refresh_set_cookie_string("jwt-here", 3600)
            .expect("set");
        assert!(
            clear.to_lowercase().contains("max-age=0"),
            "clear: {clear}"
        );
        assert!(clear.to_lowercase().contains("httponly"));
        assert!(clear.contains("refresh_token="));
        assert!(clear.contains("Path=/auth"));
        assert!(clear.to_lowercase().contains("secure"));
        assert!(clear.to_lowercase().contains("samesite=lax"));
        assert_eq!(
            clear.matches("Path=").count(),
            1,
            "single Path= like set-cookie: {clear}"
        );
        assert_eq!(set.matches("Path=").count(), 1);
    }
}
