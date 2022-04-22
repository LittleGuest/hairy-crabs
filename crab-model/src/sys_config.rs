use crab_lib::redis::{self, FromRedisValue};
use rbatis::crud_table;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 参数配置表
#[crud_table]
#[derive(Default, Clone, Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysConfig {
    /// 参数名称
    #[validate(length(max = 100))]
    pub config_name: Option<String>,
    /// 参数键名
    #[validate(length(max = 100))]
    pub config_key: Option<String>,
    /// 参数键值
    #[validate(length(max = 500))]
    pub config_value: Option<String>,
    /// 系统内置（Y是 N否）
    #[validate(length(max = 1))]
    pub config_type: Option<String>,
    /// 创建者
    #[validate(length(max = 64))]
    pub create_by: Option<String>,
    /// 创建时间
    pub create_time: Option<rbatis::DateTimeNative>,
    /// 更新者
    #[validate(length(max = 64))]
    pub update_by: Option<String>,
    /// 更新时间
    pub update_time: Option<rbatis::DateTimeNative>,
    /// 备注
    #[validate(length(max = 500))]
    pub remark: Option<String>,
    /// 岗位ID
    #[validate(length(max = 64))]
    pub id: Option<String>,
    /// 版本
    pub version: Option<i32>,
    /// 删除标志（0代表存在 1代表删除）
    #[validate(length(max = 1))]
    pub del_flag: Option<String>,
    /// 更新IP
    #[validate(length(max = 128))]
    pub update_ip: Option<String>,
    /// 创建部门
    #[validate(length(max = 64))]
    pub create_dept: Option<String>,
}

impl FromRedisValue for SysConfig {
    fn from_redis_value(_v: &redis::Value) -> redis::RedisResult<Self> {
        todo!()
    }
}
