use crab_admin::{auth, middleware, tool_gen};
use crab_config::APP;
use crab_lib::anyhow;
use poem::{get, listener::TcpListener, EndpointExt, Route, Server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    crab_model::init_db().await?;

    let route = Route::new()
        .at("/api/getInfo/:user_id", get(auth::user_info))
        .at("/api/getRouters/:user_id", get(auth::routers))
        .at("/api/tool/gen/list", get(tool_gen::gen_list))
        .around(middleware::token_middleware)
        .with(middleware::LogMilldeware);

    let listener = TcpListener::bind(format!("{}:{}", APP.server.host, APP.server.port));
    let server = Server::new(listener);
    server.run(route).await?;
    Ok(())
}
