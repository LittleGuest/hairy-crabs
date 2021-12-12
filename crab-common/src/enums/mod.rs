//!

use std::collections::HashMap;

/// 操作状态
pub enum BusinessStatus {
    /// 成功
    SUCCESS,
    /// 失败
    FAIL,
}

/// 业务操作类型
pub enum BusinessType {
    /// 其它
    OTHER,
    /// 新增
    INSERT,
    /// 修改
    UPDATE,
    /// 删除
    DELETE,
    /// 授权
    GRANT,
    /// 导出
    EXPORT,
    /// 导入
    IMPORT,
    /// 强退
    FORCE,
    /// 生成代码
    GENCODE,
    /// 清空数据
    CLEAN,
    /// 清空数据
    SELECT,
    /// 校验
    CHECK,
}

/// 数据源
pub enum DataSourceType {
    /// 主库
    MASTER,
    /// 从库
    SLAVE,
}

/// 请求方式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    TRACE,
}

const fn http_method() -> [(&'static str, HttpMethod); 8] {
    [
        ("GET", HttpMethod::GET),
        ("HEAD", HttpMethod::HEAD),
        ("POST", HttpMethod::POST),
        ("PUT", HttpMethod::PUT),
        ("PATCH", HttpMethod::PATCH),
        ("DELETE", HttpMethod::DELETE),
        ("OPTIONS", HttpMethod::OPTIONS),
        ("TRACE", HttpMethod::TRACE),
    ]
}

impl HttpMethod {
    /// 解析目标请求方式
    pub fn resolve(method: &str) -> Option<Self> {
        match http_method()
            .iter()
            .find(|(m, hm)| m.eq_ignore_ascii_case(&method))
        {
            Some(&hm) => Some(hm.1),
            None => None,
        }
    }

    /// 匹配目标请求方式是否满足
    pub fn matches(method: &str) -> bool {
        Self::resolve(method).is_some()
    }
}

/// 限流类型
pub enum LimitType {
    /// 默认策略全局限流
    DEFAULT,
    /// 根据请求者IP进行限流
    IP,
}

/// 操作人类别
pub enum OperatorType {
    /// 其它
    OTHER,
    /// 后台用户
    MANAGE,
    /// 手机端用户
    MOBILE,
}

/// 用户状态
pub struct UserStatus((u8, &'static str));

/// 用户状态宏
macro_rules! user_status {
    (
        $(
            ($code:expr, $konst:ident, $info:expr),
        )+
    ) => {
        impl UserStatus {
        $(
            pub const $konst: UserStatus = UserStatus(($code, $info));
        )+
        }

        fn info(code: u8) -> Option<&'static str> {
            match code {
                $(
                $code => Some($info),
                )+
                _ => None
            }
        }
    }
}

impl UserStatus {
    /// 获取码
    pub fn code(self) -> u8 {
        self.0 .0
    }

    /// 获取码对应的信息
    pub fn info(self) -> Option<&'static str> {
        info(self.code())
    }
}

user_status! {
    (0, OK, "正常"),
    (1, DISABLE,"停用"),
    (2, DELETED, "删除"),
}

#[cfg(test)]
mod test {
    use super::{HttpMethod, UserStatus};

    #[test]
    fn test_http_method() {
        assert_eq!(Some(HttpMethod::GET), HttpMethod::resolve("get"));
        assert_eq!(Some(HttpMethod::GET), HttpMethod::resolve("GET"));
        assert_eq!(Some(HttpMethod::GET), HttpMethod::resolve("gEt"));

        assert_eq!(Some(HttpMethod::TRACE), HttpMethod::resolve("trace"));
        assert_eq!(Some(HttpMethod::TRACE), HttpMethod::resolve("TRACE"));
        assert_eq!(Some(HttpMethod::TRACE), HttpMethod::resolve("trACE"));

        assert_eq!(true, HttpMethod::matches("get"));
        assert_eq!(true, HttpMethod::matches("GET"));
        assert_eq!(true, HttpMethod::matches("gEt"));

        assert_eq!(true, HttpMethod::matches("trace"));
        assert_eq!(true, HttpMethod::matches("TRACE"));
        assert_eq!(true, HttpMethod::matches("trACE"));

        assert_eq!(false, HttpMethod::matches("HTTP"));
    }

    #[test]
    fn test_user_status() {
        assert_eq!(0, UserStatus::OK.code());
        assert_eq!(Some("正常"), UserStatus::OK.info());

        assert_eq!(2, UserStatus::DELETED.code());
        assert_eq!(Some("删除"), UserStatus::DELETED.info());
    }
}
