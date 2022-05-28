use crab_common::{error::CrabError, result::CrabResult};
use crab_lib::{log, rbatis::Page, validator::Validate};
use crab_model::{Mapper, SysRole, SysRoleReq};

pub struct RoleSrv;

impl RoleSrv {
    pub async fn list(&self) -> CrabResult<Vec<SysRole>> {
        SysRole::list().await
    }

    pub async fn page(&self, req: SysRoleReq) -> CrabResult<Page<SysRole>> {
        SysRole::page(&req).await
    }

    pub async fn get_by_id(&self, req: SysRoleReq) -> CrabResult<Option<SysRole>> {
        if let Some(id) = req.id {
            SysRole::fetch_by_id(id).await
        } else {
            Ok(None)
        }
    }

    pub async fn save(&self, mut req: SysRole) -> CrabResult<SysRole> {
        req.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        req.id = req.save().await?;
        Ok(req)
    }

    pub async fn update(&self, req: SysRole) -> CrabResult<SysRole> {
        req.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        req.update().await?;
        Ok(req)
    }

    pub async fn delete(&self, req: SysRoleReq) -> CrabResult<u64> {
        if let Some(uid) = req.id {
            SysRole::remove_by_id(uid).await
        } else {
            Ok(0)
        }
    }

    pub async fn delete_batch(&self, ids: &[i64]) -> CrabResult<u64> {
        SysRole::remove_batch_by_ids(ids).await
    }

    pub async fn refresh_cache(&self) -> CrabResult<u64> {
        // 删除缓存，重新加载缓存
        todo!()
    }
}
