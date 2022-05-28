use crab_admin::{
    config, dict_data, dict_type, login_log, menu, middleware, oper_log, role, tool_gen, user,
    user_role,
};
use crab_config::APP;
use crab_lib::anyhow;
use poem::{listener::TcpListener, post, EndpointExt, Route, Server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    crab_model::init_db().await?;

    let route = Route::new()
        .at("/login", post(user::login))
        //
        .at("/api/user/getInfo", post(user::user_info))
        .at("/api/user/getRouters", post(user::routers))
        .at("/api/user/page", post(user::page))
        .at("/api/user/save", post(user::save))
        .at("/api/user/update", post(user::update))
        .at("/api/user/updateBatch", post(user::update_batch))
        .at("/api/user/delete", post(user::delete))
        .at("/api/user/deleteBatch", post(user::delete_batch))
        .at("/api/user/resetPwd", post(user::reset_pwd))
        .at("/api/user/import", post(user::import))
        .at("/api/user/export", post(user::export))
        // 参数配置
        .at("/api/config/list", post(config::list))
        .at("/api/config/page", post(config::page))
        .at("/api/config/getById", post(config::get_by_id))
        .at("/api/config/save", post(config::save))
        .at("/api/config/update", post(config::update))
        .at("/api/config/delete", post(config::delete))
        .at("/api/config/deleteBatch", post(config::delete_batch))
        .at("/api/config/refreshCache", post(config::refresh_cache))
        .at("/api/config/export", post(config::export))
        // 字典类型
        .at("/api/dictType/list", post(dict_type::list))
        .at("/api/dictType/page", post(dict_type::page))
        .at("/api/dictType/getById", post(dict_type::get_by_id))
        .at("/api/dictType/save", post(dict_type::save))
        .at("/api/dictType/update", post(dict_type::update))
        .at("/api/dictType/delete", post(dict_type::delete))
        .at("/api/dictType/deleteBatch", post(dict_type::delete_batch))
        .at("/api/dictType/refreshCache", post(dict_type::refresh_cache))
        .at("/api/dictType/export", post(dict_type::export))
        // 字典数据
        .at("/api/dictData/list", post(dict_data::list))
        .at("/api/dictData/page", post(dict_data::page))
        .at("/api/dictData/getById", post(dict_data::get_by_id))
        .at("/api/dictData/save", post(dict_data::save))
        .at("/api/dictData/update", post(dict_data::update))
        .at("/api/dictData/delete", post(dict_data::delete))
        .at("/api/dictData/deleteBatch", post(dict_data::delete_batch))
        .at("/api/dictData/refreshCache", post(dict_data::refresh_cache))
        .at("/api/dictData/export", post(dict_data::export))
        // 菜单配置
        .at("/api/menu/list", post(menu::list))
        .at("/api/menu/page", post(menu::page))
        .at("/api/menu/getById", post(menu::get_by_id))
        .at("/api/menu/save", post(menu::save))
        .at("/api/menu/update", post(menu::update))
        .at("/api/menu/delete", post(menu::delete))
        .at("/api/menu/deleteBatch", post(menu::delete_batch))
        .at("/api/menu/refreshCache", post(menu::refresh_cache))
        .at("/api/menu/export", post(menu::export))
        // 角色管理
        .at("/api/role/list", post(role::list))
        .at("/api/role/page", post(role::page))
        .at("/api/role/getById", post(role::get_by_id))
        .at("/api/role/save", post(role::save))
        .at("/api/role/update", post(role::update))
        .at("/api/role/delete", post(role::delete))
        .at("/api/role/deleteBatch", post(role::delete_batch))
        .at("/api/role/refreshCache", post(role::refresh_cache))
        .at("/api/role/export", post(role::export))
        // 用户角色
        .at("/api/userRole/saveBatch", post(user_role::save_batch))
        .at("/api/userRole/delete", post(user_role::delete))
        // 操作日志
        .at("/api/log/page", post(oper_log::page))
        .at("/api/log/getById", post(oper_log::get_by_id))
        .at("/api/log/clear", post(oper_log::clear))
        // 操作日志
        .at("/api/loginLog/page", post(login_log::page))
        .at("/api/loginLog/getById", post(login_log::get_by_id))
        .at("/api/loginLog/clear", post(login_log::clear))
        //
        .at("/api/tool/gen/list", post(tool_gen::gen_list))
        .around(middleware::auth_middleware)
        .around(middleware::token_middleware)
        .with(middleware::LogMilldeware);

    let listener = TcpListener::bind(format!("{}:{}", APP.server.host, APP.server.port));
    let server = Server::new(listener);
    server.run(route).await?;
    Ok(())
}
