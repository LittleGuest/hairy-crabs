use poem::{handler, web::Data, Endpoint, EndpointExt, Middleware, Request, Result};

/// jwt 中间件
struct TokenMiddleware;

impl<E: Endpoint> Middleware<E> for TokenMiddleware {
    type Output = TokenMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        TokenMiddlewareImpl { ep }
    }
}

struct TokenMiddlewareImpl<E> {
    ep: E,
}

const TOKEN_HEADER: &str = "X-Token";

/// Token data
struct Token(String);

#[poem::async_trait]
impl<E: Endpoint> Endpoint for TokenMiddlewareImpl<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        if let Some(value) = req
            .headers()
            .get(TOKEN_HEADER)
            .and_then(|value| value.to_str().ok())
        {
            let token = value.to_string();
            req.extensions_mut().insert(Token(token));
        }

        self.ep.call(req).await
    }
}
