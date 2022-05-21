use crab_admin::{middleware, tool_gen, user};
use crab_config::APP;
use crab_lib::anyhow;
use poem::{listener::TcpListener, post, EndpointExt, Route, Server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    crab_model::init_db().await?;

    let route = Route::new()
        .at("/login", post(user::login))
        .at("/api/user/getInfo", post(user::user_info))
        .at("/api/user/getRouters", post(user::routers))
        .at("/api/user/page", post(user::page))
        .at("/api/tool/gen/list", post(tool_gen::gen_list))
        .around(middleware::auth_middleware)
        .around(middleware::token_middleware)
        .with(middleware::LogMilldeware);

    let listener = TcpListener::bind(format!("{}:{}", APP.server.host, APP.server.port));
    let server = Server::new(listener);
    server.run(route).await?;
    Ok(())
}
