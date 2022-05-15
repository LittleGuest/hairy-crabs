use poem::{async_trait, Endpoint, IntoResponse, Middleware, Request, Response, Result};

pub struct LogMilldeware;

impl<E: Endpoint> Middleware<E> for LogMilldeware {
    type Output = LogImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        LogImpl(ep)
    }
}

pub struct LogImpl<E>(E);

#[async_trait]
impl<E: Endpoint> Endpoint for LogImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        println!("请求: {}", req.uri().path());
        let res = self.0.call(req).await;

        match res {
            Ok(resp) => {
                let resp = resp.into_response();
                println!("响应: {}", resp.status());
                Ok(resp)
            }
            Err(err) => {
                println!("错误: {}", err);
                Err(err)
            }
        }
    }
}
