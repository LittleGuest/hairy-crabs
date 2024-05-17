use crab_common::{error::CrabError, result::CrabResult};
use crab_lib::{log, rbatis::Page, validator::Validate};
use crab_model::{Mapper, SysLog, SysLogReq, SysLoginLog, SysLoginLogReq};

pub struct LogSrv;

impl LogSrv {
    pub async fn page(&self, req: SysLogReq) -> CrabResult<Page<SysLog>> {
        SysLog::page(&req).await
    }

    pub async fn get_by_id(&self, req: SysLogReq) -> CrabResult<Option<SysLog>> {
        if let Some(id) = req.id {
            SysLog::fetch_by_id(id).await
        } else {
            Ok(None)
        }
    }

    pub async fn save(&self, mut req: SysLog) -> CrabResult<SysLog> {
        req.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        req.id = req.save().await?;
        Ok(req)
    }

    pub async fn clear(&self) -> CrabResult<u64> {
        todo!()
    }
}

pub struct LoginLogSrv;

impl LoginLogSrv {
    pub async fn page(&self, req: SysLoginLogReq) -> CrabResult<Page<SysLoginLog>> {
        SysLoginLog::page(&req).await
    }

    pub async fn get_by_id(&self, req: SysLoginLogReq) -> CrabResult<Option<SysLoginLog>> {
        if let Some(id) = req.id {
            SysLoginLog::fetch_by_id(id).await
        } else {
            Ok(None)
        }
    }

    pub async fn save(&self, mut req: SysLoginLog) -> CrabResult<SysLoginLog> {
        req.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        req.id = req.save().await?;
        Ok(req)
    }

    pub async fn clear(&self) -> CrabResult<u64> {
        todo!()
    }
}
