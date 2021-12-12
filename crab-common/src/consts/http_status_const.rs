//! 返回状态码

/// 操作成功
pub const SUCCESS: u32 = 200;
/// 对象创建成功
pub const CREATED: u32 = 201;
/// 请求已经被接受
pub const ACCEPTED: u32 = 202;
/// 操作已经执行成功，但是没有返回数据
pub const NO_CONTENT: u32 = 204;
/// 资源已被移除
pub const MOVED_PERM: u32 = 301;
/// 重定向
pub const SEE_OTHER: u32 = 303;
/// 资源没有被修改
pub const NOT_MODIFIED: u32 = 304;
/// 参数列表错误（缺少，格式不匹配）
pub const BAD_REQUEST: u32 = 400;
/// 未授权
pub const UNAUTHORIZED: u32 = 401;
/// 访问受限，授权过期
pub const FORBIDDEN: u32 = 403;
/// 资源，服务未找到
pub const NOT_FOUND: u32 = 404;
/// 不允许的http方法
pub const BAD_METHOD: u32 = 405;
/// 资源冲突，或者资源被锁
pub const CONFLICT: u32 = 409;
/// 不支持的数据，媒体类型
pub const UNSUPPORTED_TYPE: u32 = 415;
/// 系统内部错误
pub const ERROR: u32 = 500;
/// 接口未实现
pub const NOT_IMPLEMENTED: u32 = 501;
/// 失败标记
pub const FAIL: u32 = 500;
