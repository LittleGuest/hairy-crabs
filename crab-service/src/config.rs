use crab_common::{error::CrabError, result::CrabResult};
use crab_lib::{log, rbatis::Page, validator::Validate};
use crab_model::{ConfigReq, Mapper, SysConfig};

pub struct ConfigSrv;

impl ConfigSrv {
    pub async fn list(&self) -> CrabResult<Vec<SysConfig>> {
        SysConfig::list().await
    }

    pub async fn page(&self, req: ConfigReq) -> CrabResult<Page<SysConfig>> {
        SysConfig::page(&req).await
    }

    pub async fn get_by_id(&self, req: ConfigReq) -> CrabResult<Option<SysConfig>> {
        if let Some(id) = req.id {
            SysConfig::fetch_by_id(id).await
        } else {
            Ok(None)
        }
    }

    pub async fn save(&self, mut req: SysConfig) -> CrabResult<SysConfig> {
        req.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        req.id = req.save().await?;
        Ok(req)
    }

    pub async fn update(&self, req: SysConfig) -> CrabResult<SysConfig> {
        req.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        req.update().await?;
        Ok(req)
    }

    pub async fn delete(&self, req: ConfigReq) -> CrabResult<u64> {
        if let Some(uid) = req.id {
            SysConfig::remove_by_id(uid).await
        } else {
            Ok(0)
        }
    }

    pub async fn delete_batch(&self, ids: &[i64]) -> CrabResult<u64> {
        SysConfig::remove_batch_by_ids(ids).await
    }

    pub async fn refresh_cache(&self) -> CrabResult<u64> {
        // 删除缓存，重新加载缓存
        todo!()
    }
}
