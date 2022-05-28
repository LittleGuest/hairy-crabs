use crab_admin::{
    config, dict_data, dict_type, gen_config_template, gen_table, gen_table_column, login_log,
    menu, middleware, oper_log, role, user, user_role,
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
        // 用户管理
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
        // 模板配置
        .at("/api/gen/template/list", post(gen_config_template::list))
        .at("/api/gen/template/page", post(gen_config_template::page))
        .at(
            "/api/gen/template/getById",
            post(gen_config_template::get_by_id),
        )
        .at("/api/gen/template/save", post(gen_config_template::save))
        .at(
            "/api/gen/template/update",
            post(gen_config_template::update),
        )
        .at(
            "/api/gen/template/delete",
            post(gen_config_template::delete),
        )
        .at(
            "/api/gen/template/deleteBatch",
            post(gen_config_template::delete_batch),
        )
        .at(
            "/api/gen/template/changeStatus",
            post(gen_config_template::change_status),
        )
        .at(
            "/api/gen/template/changeTemplateDefault",
            post(gen_config_template::change_template_default),
        )
        // 代码生成
        .at("/api/gen/table/list", post(gen_table::list))
        .at("/api/gen/table/page", post(gen_table::page))
        .at("/api/gen/table/getById", post(gen_table::get_by_id))
        .at("/api/gen/table/save", post(gen_table::save))
        .at("/api/gen/table/update", post(gen_table::update))
        .at("/api/gen/table/delete", post(gen_table::delete))
        .at("/api/gen/table/deleteBatch", post(gen_table::delete_batch))
        // 代码生成
        .at("/api/gen/tableColumn/list", post(gen_table_column::list))
        .at("/api/gen/tableColumn/page", post(gen_table_column::page))
        .at(
            "/api/gen/tableColumn/getById",
            post(gen_table_column::get_by_id),
        )
        .at("/api/gen/tableColumn/save", post(gen_table_column::save))
        .at(
            "/api/gen/tableColumn/update",
            post(gen_table_column::update),
        )
        .at(
            "/api/gen/tableColumn/delete",
            post(gen_table_column::delete),
        )
        .at(
            "/api/gen/tableColumn/deleteBatch",
            post(gen_table_column::delete_batch),
        )
        .around(middleware::auth_middleware)
        .around(middleware::token_middleware)
        .with(middleware::LogMilldeware);

    let listener = TcpListener::bind(format!("{}:{}", APP.server.host, APP.server.port));
    let server = Server::new(listener);
    server.run(route).await?;
    Ok(())
}
