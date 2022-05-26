use crab_common::{error::CrabError, result::CrabResult};
use crab_lib::{log, rbatis::Page, validator::Validate};
use crab_model::{DictDataReq, DictTypeReq, Mapper, SysDictData, SysDictType};

pub struct DictTypeSrv;

impl DictTypeSrv {
    pub async fn list(&self) -> CrabResult<Vec<SysDictType>> {
        SysDictType::list().await
    }

    pub async fn page(&self, req: DictTypeReq) -> CrabResult<Page<SysDictType>> {
        SysDictType::page(&req).await
    }

    pub async fn get_by_id(&self, req: DictTypeReq) -> CrabResult<Option<SysDictType>> {
        if let Some(id) = req.id {
            SysDictType::fetch_by_id(id).await
        } else {
            Ok(None)
        }
    }

    pub async fn save(&self, mut req: SysDictType) -> CrabResult<SysDictType> {
        req.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        req.id = req.save().await?;
        Ok(req)
    }

    pub async fn update(&self, req: SysDictType) -> CrabResult<SysDictType> {
        req.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        req.update().await?;
        Ok(req)
    }

    pub async fn delete(&self, req: DictTypeReq) -> CrabResult<u64> {
        if let Some(uid) = req.id {
            // TODO 不删除有子的类型
            SysDictType::remove_by_id(uid).await
        } else {
            Ok(0)
        }
    }

    pub async fn delete_batch(&self, ids: &[i64]) -> CrabResult<u64> {
        // TODO 不删除有子的类型
        SysDictType::remove_batch_by_ids(ids).await
    }

    pub async fn refresh_cache(&self) -> CrabResult<u64> {
        // 删除缓存，重新加载缓存
        todo!()
    }
}

pub struct DictDataSrv;

impl DictDataSrv {
    pub async fn list(&self) -> CrabResult<Vec<SysDictData>> {
        SysDictData::list().await
    }

    pub async fn page(&self, req: DictDataReq) -> CrabResult<Page<SysDictData>> {
        SysDictData::page(&req).await
    }

    pub async fn get_by_id(&self, req: DictDataReq) -> CrabResult<Option<SysDictData>> {
        if let Some(id) = req.id {
            SysDictData::fetch_by_id(id).await
        } else {
            Ok(None)
        }
    }

    pub async fn save(&self, mut req: SysDictData) -> CrabResult<SysDictData> {
        req.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        req.id = req.save().await?;
        Ok(req)
    }

    pub async fn update(&self, req: SysDictData) -> CrabResult<SysDictData> {
        req.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        req.update().await?;
        Ok(req)
    }

    pub async fn delete(&self, req: DictDataReq) -> CrabResult<u64> {
        if let Some(uid) = req.id {
            SysDictData::remove_by_id(uid).await
        } else {
            Ok(0)
        }
    }

    pub async fn delete_batch(&self, ids: &[i64]) -> CrabResult<u64> {
        SysDictData::remove_batch_by_ids(ids).await
    }

    pub async fn refresh_cache(&self) -> CrabResult<u64> {
        // 删除缓存，重新加载缓存
        todo!()
    }

    pub async fn get_by_type(&self, req: DictDataReq) -> CrabResult<Vec<SysDictData>> {
        let datas = self.list().await?;
        Ok(datas
            .into_iter()
            .filter(|d| d.dict_type.eq(&req.dict_type))
            .collect::<Vec<_>>())
    }
}
