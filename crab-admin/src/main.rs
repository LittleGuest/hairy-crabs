use crab_admin::{auth, tool_gen};
use crab_config::APP;
use crab_lib::anyhow;
use poem::{get, listener::TcpListener, post, Route, Server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    crab_model::init_db().await?;

    let route = Route::new()
        .at("/api/login", post(auth::login))
        .at("/api/getInfo/:user_id", get(auth::user_info))
        .at("/api/getRouters/:user_id", get(auth::routers))
        .at("/api/tool/gen/list", get(tool_gen::gen_list));

    let listener = TcpListener::bind(format!("{}:{}", APP.server.host, APP.server.port));
    let server = Server::new(listener);
    server.run(route).await?;
    Ok(())
}
