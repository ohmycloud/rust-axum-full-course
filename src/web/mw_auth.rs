use axum::extract::{FromRequestParts, Request, State};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;
use crate::{error::Error, error::Result};

impl<S> FromRequestParts<S> for Ctx
where
    S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}

pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Compute Result<Ctx>
    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => Ok(Ctx::new(user_id)),
        Err(err) => Err(err),
    };

    // Remove the cookie if something went wrong other than NoAuthTokenCookie
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    // Store the ctx in the request extensions
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

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
