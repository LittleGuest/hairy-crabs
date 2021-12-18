//! 通用常量信息

pub mod config;
pub mod gen_const;
pub mod http_status_const;
pub mod schedule_const;
pub mod use_const;

/// UTF-8 字符集    
pub const UTF8: &str = "UTF-8";
/// GBK 字符集
pub const GBK: &str = "GBK";
/// http请求
pub const HTTP: &str = "http://";
/// https请求
pub const HTTPS: &str = "https://";
/// 通用成功标识
pub const SUCCESS: &str = "0";
/// 通用失败标识
pub const FAIL: &str = "1";
/// 登录成功
pub const LOGIN_SUCCESS: &str = "Success";
/// 注销
pub const LOGOUT: &str = "Logout";
/// 注册
pub const REGISTER: &str = "Register";
/// 登录失败
pub const LOGIN_FAIL: &str = "Error";
/// 验证码 redis key
pub const CAPTCHA_CODE_KEY: &str = "captcha_codes:";
/// 登录用户 redis key
pub const LOGIN_TOKEN_KEY: &str = "login_tokens:";
/// 防重提交 redis key
pub const REPEAT_SUBMIT_KEY: &str = "repeat_submit:";
/// 限流 redis key
pub const RATE_LIMIT_KEY: &str = "rate_limit:";
/// 验证码有效期（分钟）
pub const CAPTCHA_EXPIRATION: u8 = 2;
/// 令牌
pub const TOKEN: &str = "token";
/// 令牌前缀
pub const TOKEN_PREFIX: &str = "Bearer ";
/// 令牌前缀
pub const LOGIN_USER_KEY: &str = "login_user_key";
/// 用户ID
pub const JWT_USERID: &str = "userid";
/// 用户名称
// pub const JWT_USERNAME: &str = Claims.SUBJECT;
/// 用户头像
pub const JWT_AVATAR: &str = "avatar";
/// 创建时间
pub const JWT_CREATED: &str = "created";
/// 用户权限
pub const JWT_AUTHORITIES: &str = "authorities";
/// 参数管理 cache key
pub const SYS_CONFIG_KEY: &str = "sys_config:";
/// 字典管理 cache key
pub const SYS_DICT_KEY: &str = "sys_dict:";
/// 用户管理 cache key
pub const SYS_USER_KEY: &str = "sys_user:";
/// 用户管理 cache key
pub const SYS_USER_UN_KEY: &str = "sys_user_un:";
/// 部门管理 cache key
pub const SYS_DEPT_KEY: &str = "sys_dept:";
/// 列配置 cache key
pub const SYS_TABLE_CONFIG_KEY: &str = "sys_table_config:";
/// 部门管理 cache key
pub const SYS_DEPT_DC_KEY: &str = "sys_dept_dc:";
/// 角色管理 cache key
pub const SYS_ROLE_KEY: &str = "sys_role:";
/// RMI 远程方法调用
pub const LOOKUP_RMI: &str = "rmi://";

/// LDAP 远程方法调用
pub const LOOKUP_LDAP: &str = "ldap://";
/// 定时任务违规的字符
pub const JOB_ERROR_STR: [&str; 4] = [
    "java.net.URL",
    "javax.naming.InitialContext",
    "org.yaml.snakeyaml",
    "org.springframework.jndi",
];
/// 资源映射路径 前缀
pub const RESOURCE_PREFIX: &str = "/profile";
/// 默认为空消息
pub const DEFAULT_NULL_MESSAGE: &str = "暂无数据";
/// 默认成功消息
pub const DEFAULT_SUCCESS_MESSAGE: &str = "操作成功";
/// 默认失败消息
pub const DEFAULT_FAILURE_MESSAGE: &str = "操作失败";
/// 树的ID串分隔符
pub const TREE_ROOT: &str = "0";
/// 树的ID串分隔符
pub const TREE_IDS_SPLIT_CHART: &str = "/";
/// 树的ID串分隔符
pub const TREE_LEAF_Y: &str = "y";
/// 树的ID串分隔符
pub const TREE_LEAF_N: &str = "n";
/// 日志操作类型
pub enum OpType {
    Login,
    Insert,
    Delete,
    Update,
    Select,
    Logout,
}
/// 登录用户编号 redis key
pub const LOGIN_USERID_KEY: &str = "login_userid:";
/// 注册
pub const ATTACH_SAVE_TYPE_DISK: &str = "Disk";
