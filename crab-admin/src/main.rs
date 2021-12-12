use crab_admin::RB;
use poem::{listener::TcpListener, Route, Server};
use rbatis::db::DBPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_db().await?;
    start().await?;
    Ok(())
}

/// 启动应用
async fn start() -> anyhow::Result<()> {
    // let api_service = OpenApiService::new(Api, "hello api   ", "0.1")
    //     .server_with_description("127.0.0.1:8080/api", "server description");
    // let ui = api_service.swagger_ui();
    // let route = Route::new().at("/api", api_service).nest("/", ui);
    let route = Route::new();

    let listener = TcpListener::bind("127.0.0.1:8080");
    let server = Server::new(listener);
    server.run(route).await?;
    Ok(())
}

/// 初始化数据库
async fn init_db() -> anyhow::Result<()> {
    //启用日志输出
    fast_log::init_log("hairy_crabs.log", 1000, log::Level::Info, None, true).unwrap();
    //初始化连接池
    let pool_options = DBPoolOptions::default();
    RB.link_opt("mysql://root:root@localhost:3306/ry-vue", pool_options)
        .await?;
    Ok(())
}
