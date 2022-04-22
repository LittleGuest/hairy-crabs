use rbatis::crud_table;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 用户信息表
#[crud_table]
#[derive(Default, Clone, Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysUser {
    /// 用户ID
    #[validate(length(max = 64))]
    pub id: Option<String>,
    /// 部门ID
    #[validate(length(max = 50))]
    pub dept_id: Option<String>,
    /// 姓名
    #[validate(length(max = 50))]
    pub name: Option<String>,
    /// 英文名
    #[validate(length(max = 50))]
    pub name_en: Option<String>,
    /// 用户编号
    #[validate(length(max = 50))]
    pub no: Option<String>,
    /// 登陆名称
    #[validate(length(max = 30))]
    pub user_name: Option<String>,
    /// 别称
    #[validate(length(max = 30))]
    pub nick_name: Option<String>,
    /// 前后台类型标识 0:后台用户 1:前台用户
    #[validate(length(max = 2))]
    pub user_type: Option<String>,
    /// 用户邮箱
    #[validate(length(max = 50))]
    pub email: Option<String>,
    /// 手机号码
    #[validate(length(max = 11))]
    pub phonenumber: Option<String>,
    /// 用户性别（0男 1女 2未知）
    #[validate(length(max = 1))]
    pub sex: Option<String>,
    /// 头像地址
    #[validate(length(max = 100))]
    pub avatar: Option<String>,
    /// 密码
    #[validate(length(max = 100))]
    pub password: Option<String>,
    /// 生日
    pub birthday: Option<rbatis::DateNative>,
    /// 民族
    #[validate(length(max = 100))]
    pub nation: Option<String>,
    /// 籍贯
    #[validate(length(max = 100))]
    pub birth_address: Option<String>,
    /// 政治面貌
    #[validate(length(max = 50))]
    pub polity: Option<String>,
    /// 职称
    #[validate(length(max = 100))]
    pub title: Option<String>,
    /// 办公电话
    #[validate(length(max = 50))]
    pub office_tel: Option<String>,
    /// 传真号
    #[validate(length(max = 50))]
    pub fax: Option<String>,
    /// 工作地点
    #[validate(length(max = 100))]
    pub work_space: Option<String>,
    /// 排序号
    pub sort: Option<i32>,
    /// 用户姓名全拼和简拼
    #[validate(length(max = 500))]
    pub user_pinyin: Option<String>,
    /// 帐号状态（0正常 1停用）
    #[validate(length(max = 1))]
    pub status: Option<String>,
    /// 删除标志（0代表存在 2代表删除）
    #[validate(length(max = 1))]
    pub del_flag: Option<String>,
    /// 最后登录IP
    #[validate(length(max = 128))]
    pub login_ip: Option<String>,
    /// 最后登录时间
    pub login_date: Option<rbatis::DateTimeNative>,
    /// 创建者
    #[validate(length(max = 64))]
    pub create_by: Option<String>,
    /// 创建部门
    #[validate(length(max = 64))]
    pub create_dept: Option<String>,
    /// 创建时间
    pub create_time: Option<rbatis::DateTimeNative>,
    /// 更新者
    #[validate(length(max = 64))]
    pub update_by: Option<String>,
    /// 更新时间
    pub update_time: Option<rbatis::DateTimeNative>,
    /// 更新IP
    #[validate(length(max = 128))]
    pub update_ip: Option<String>,
    /// 备注
    #[validate(length(max = 500))]
    pub remark: Option<String>,
    /// 版本
    pub version: Option<i32>,
}
