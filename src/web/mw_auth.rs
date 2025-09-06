use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;
use crate::{error::Error, error::Result};

pub async fn mw_require_auth(cookies: Cookies, req: Request, next: Next) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;

    Ok(next.run(req).await)
}
