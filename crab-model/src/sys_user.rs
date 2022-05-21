use std::collections::HashSet;

use crab_common::{error::CrabError, result::CrabResult, PageDto};
use rbatis::{crud::CRUD, crud_table, Page, PageRequest};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{Mapper, RB};

/// 用户信息表
#[crud_table]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysUser {
    /// 用户ID
    pub id: Option<i64>,
    /// 用户编号
    #[validate(length(max = 50))]
    pub no: Option<String>,
    /// 姓名
    #[validate(length(max = 50))]
    pub name: Option<String>,
    /// 帐号
    #[validate(length(max = 30))]
    pub account: Option<String>,
    /// 密码
    #[validate(length(max = 100))]
    pub password: Option<String>,
    /// 邮箱
    #[validate(length(max = 50))]
    pub email: Option<String>,
    /// 手机号码
    #[validate(length(max = 11))]
    pub phone: Option<String>,
    /// 性别（0男 1女 2未知）
    #[validate(length(max = 1))]
    pub sex: Option<String>,
    /// 头像地址
    #[validate(length(max = 100))]
    pub avatar: Option<String>,
    /// 帐号状态（0正常 1停用）
    pub status: Option<i8>,
    /// 最后登录IP
    #[validate(length(max = 128))]
    pub login_ip: Option<String>,
    /// 最后登录时间
    pub login_at: Option<rbatis::DateTimeNative>,
    /// 备注
    #[validate(length(max = 500))]
    pub remark: Option<String>,
    /// 创建者
    pub create_by: Option<i64>,
    /// 创建时间
    pub create_at: Option<rbatis::DateTimeNative>,
    /// 更新者
    pub update_by: Option<i64>,
    /// 更新时间
    pub update_at: Option<rbatis::DateTimeNative>,
    /// 删除标志（0代表存在 1代表删除）
    pub del_flag: Option<i8>,
}

impl std::fmt::Display for SysUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::json!(self))
    }
}

#[crab_lib::async_trait::async_trait]
impl Mapper for SysUser {
    async fn save(&self) -> CrabResult<Option<i64>> {
        let res = RB.save(self, &[]).await.map_err(|e| {
            log::error!("Mapper::save error {}", e);
            CrabError::SqlError
        })?;
        Ok(res.last_insert_id)
    }
    async fn save_batch(models: &[Self]) -> CrabResult<u64> {
        let res = RB.save_batch(models, &[]).await.map_err(|e| {
            log::error!("Mapper::save_batch error {}", e);
            CrabError::SqlError
        })?;
        Ok(res.rows_affected)
    }
    async fn update(&self) -> CrabResult<u64> {
        let w = RB.new_wrapper().eq("id", self.id);
        let res = RB.update_by_wrapper(self, w, &[]).await.map_err(|e| {
            log::error!("Mapper::update error {}", e);
            CrabError::SqlError
        })?;
        Ok(res)
    }
    async fn remove_by_id(id: i64) -> CrabResult<u64> {
        let res = RB
            .remove_by_column::<Self, _>("id", id)
            .await
            .map_err(|e| {
                log::error!("Mapper::remove_by_id error {}", e);
                CrabError::SqlError
            })?;
        Ok(res)
    }
    async fn remove_batch_by_ids(ids: &[i64]) -> CrabResult<u64> {
        let res = RB
            .remove_batch_by_column::<Self, _>("id", ids)
            .await
            .map_err(|e| {
                log::error!("Mapper::remove_batch_by_ids error {}", e);
                CrabError::SqlError
            })?;
        Ok(res)
    }
    async fn list() -> CrabResult<Vec<Self>> {
        let res = RB.fetch_list().await.map_err(|e| {
            log::error!("Mapper::list error {}", e);
            CrabError::SqlError
        })?;
        Ok(res)
    }
    async fn fetch_by_id(id: i64) -> CrabResult<Option<Self>> {
        let res = RB.fetch_by_column("id", id).await.map_err(|e| {
            log::error!("Mapper::fetch_by_id error {}", e);
            CrabError::SqlError
        })?;
        Ok(res)
    }
    async fn fetch_by_ids(ids: &[i64]) -> CrabResult<Vec<Self>> {
        let res = RB.fetch_list_by_column("id", ids).await.map_err(|e| {
            log::error!("Mapper::fetch_by_ids error {}", e);
            CrabError::SqlError
        })?;
        Ok(res)
    }
}

impl SysUser {
    pub async fn get_by_username(account: &str) -> CrabResult<Option<Self>> {
        let user = RB.fetch_by_column("account", account).await.map_err(|e| {
            log::error!("根据用户名获取用户信息失败: {}", e);
            CrabError::SqlError
        })?;
        Ok(user)
    }

    pub async fn get_by_id(id: i64) -> CrabResult<Option<Self>> {
        let user = RB.fetch_by_column("id", id).await.map_err(|e| {
            log::error!("根据用户ID获取用户信息失败: {}", e);
            CrabError::SqlError
        })?;
        Ok(user)
    }

    pub async fn page(req: UserReq) -> CrabResult<Page<Self>> {
        let pr = PageRequest::new(req.page.page_no, req.page.page_size);
        let w = RB.new_wrapper();
        let res = RB.fetch_page_by_wrapper(w, &pr).await.map_err(|e| {
            log::error!("Mapper::fetch_by_id error {}", e);
            CrabError::SqlError
        })?;
        Ok(res)
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginBody {
    /// 用户名
    pub account: String,
    /// 密码
    pub password: String,
    /// 验证码
    pub code: String,
    /// 唯一标识
    pub uuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUserDto {
    /// 用户信息
    pub user: SysUser,
    /// jwt
    pub access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserInfoDto {
    /// 用户信息
    pub user: SysUser,
    /// 角色集合
    pub roles: HashSet<String>,
    /// 权限集合
    pub permissions: HashSet<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserReq {
    page: PageDto,
}
