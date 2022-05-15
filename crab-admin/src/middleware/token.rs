use crab_common::jwt::JWTToken;
use crab_config::APP;
use poem::{Endpoint, EndpointExt, Request, Result};

const TOKEN_HEADER: &str = "X-Token";

pub async fn token_middleware<E: Endpoint>(next: E, mut req: Request) -> Result<E::Output> {
    log::info!("token中间件");
    let uri = req.uri().path();
    // 白名单不校验token
    let wl = APP.white_list();
    if !wl.is_empty() && wl.iter().any(|u| u.starts_with(uri)) {
        if let Some(value) = req
            .headers()
            .get(TOKEN_HEADER)
            .and_then(|value| value.to_str().ok())
        {
            match JWTToken::verify(value) {
                Ok(token) => {
                    req.extensions_mut().insert(token);
                }
                Err(e) => {
                    log::error!("验证token有效失败: uri = {uri}, token = {value}, {e}");
                }
            }
        }
    }
    next.call(req).await
}
