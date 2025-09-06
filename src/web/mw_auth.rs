use async_trait::async_trait;
use axum::RequestPartsExt;
use axum::extract::{FromRequestParts, Request};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use std::future::Future;
use tower_cookies::Cookies;

use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;
use crate::{error::Error, error::Result};

#[async_trait]
impl<S> FromRequestParts<S> for Ctx
where
    S: Send + Sync,
{
    type Rejection = Error;
    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self>> + Send {
        async move {
            println!("->> {:<12} - Ctx", "EXTRACTOR");

            // use the cookies extractor
            let cookies = parts.extract::<Cookies>().await.unwrap();
            let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

            // parse token
            let (user_id, _exp, _sign) = auth_token
                .ok_or(Error::AuthFailNoAuthTokenCookie)
                .and_then(parse_token)?;

            Ok(Ctx::new(user_id))
        }
    }
}

pub async fn mw_require_auth(cookies: Cookies, req: Request, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // parse token
    let (_user_id, _exp, _sign) = auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)?;

    Ok(next.run(req).await)
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
