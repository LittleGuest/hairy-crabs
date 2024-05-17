use crab_common::{error::CrabError, result::CrabResult};
use crab_lib::{log, rbatis::Page, validator::Validate};
use crab_model::{
    GenConfigTemplate, GenConfigTemplateReq, GenTable, GenTableColumn, GenTableColumnReq,
    GenTableReq, Mapper,
};

pub struct GenConfigTemplateSrv;

impl GenConfigTemplateSrv {
    pub async fn list(&self) -> CrabResult<Vec<GenConfigTemplate>> {
        GenConfigTemplate::list().await
    }

    pub async fn page(&self, req: GenConfigTemplateReq) -> CrabResult<Page<GenConfigTemplate>> {
        GenConfigTemplate::page(&req).await
    }

    pub async fn get_by_id(
        &self,
        req: GenConfigTemplateReq,
    ) -> CrabResult<Option<GenConfigTemplate>> {
        if let Some(id) = req.id {
            GenConfigTemplate::fetch_by_id(id).await
        } else {
            Ok(None)
        }
    }

    pub async fn save(&self, mut req: GenConfigTemplate) -> CrabResult<GenConfigTemplate> {
        req.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        req.id = req.save().await?;
        Ok(req)
    }

    pub async fn update(&self, req: GenConfigTemplate) -> CrabResult<GenConfigTemplate> {
        req.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        req.update().await?;
        Ok(req)
    }

    pub async fn delete(&self, req: GenConfigTemplateReq) -> CrabResult<u64> {
        if let Some(uid) = req.id {
            GenConfigTemplate::remove_by_id(uid).await
        } else {
            Ok(0)
        }
    }

    pub async fn delete_batch(&self, ids: &[i64]) -> CrabResult<u64> {
        GenConfigTemplate::remove_batch_by_ids(ids).await
    }
}

pub struct GenTableSrv;

impl GenTableSrv {
    pub async fn list(&self) -> CrabResult<Vec<GenTable>> {
        GenTable::list().await
    }

    pub async fn page(&self, req: GenTableReq) -> CrabResult<Page<GenTable>> {
        GenTable::page(&req).await
    }

    pub async fn get_by_id(&self, req: GenTableReq) -> CrabResult<Option<GenTable>> {
        if let Some(id) = req.id {
            GenTable::fetch_by_id(id).await
        } else {
            Ok(None)
        }
    }

    pub async fn save(&self, mut req: GenTable) -> CrabResult<GenTable> {
        req.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        req.id = req.save().await?;
        Ok(req)
    }

    pub async fn update(&self, req: GenTable) -> CrabResult<GenTable> {
        req.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        req.update().await?;
        Ok(req)
    }

    pub async fn delete(&self, req: GenTableReq) -> CrabResult<u64> {
        if let Some(uid) = req.id {
            GenTable::remove_by_id(uid).await
        } else {
            Ok(0)
        }
    }

    pub async fn delete_batch(&self, ids: &[i64]) -> CrabResult<u64> {
        GenTable::remove_batch_by_ids(ids).await
    }
}

pub struct GenTableColumnSrv;

impl GenTableColumnSrv {
    pub async fn list(&self) -> CrabResult<Vec<GenTableColumn>> {
        GenTableColumn::list().await
    }

    pub async fn page(&self, req: GenTableColumnReq) -> CrabResult<Page<GenTableColumn>> {
        GenTableColumn::page(&req).await
    }

    pub async fn get_by_id(&self, req: GenTableColumnReq) -> CrabResult<Option<GenTableColumn>> {
        if let Some(id) = req.id {
            GenTableColumn::fetch_by_id(id).await
        } else {
            Ok(None)
        }
    }

    pub async fn save(&self, mut req: GenTableColumn) -> CrabResult<GenTableColumn> {
        req.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        req.id = req.save().await?;
        Ok(req)
    }

    pub async fn update(&self, req: GenTableColumn) -> CrabResult<GenTableColumn> {
        req.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        req.update().await?;
        Ok(req)
    }

    pub async fn delete(&self, req: GenTableColumnReq) -> CrabResult<u64> {
        if let Some(uid) = req.id {
            GenTableColumn::remove_by_id(uid).await
        } else {
            Ok(0)
        }
    }

    pub async fn delete_batch(&self, ids: &[i64]) -> CrabResult<u64> {
        GenTableColumn::remove_batch_by_ids(ids).await
    }
}
