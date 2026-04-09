use crate::model::sys::model::msys_white_jwt::{
    SysWhiteJwtModel, WhiteJwtDel, WhiteJwtRes, WhiteJwtSearch,
};
use crate::service::prelude::*;
pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<WhiteJwtSearch>,
) -> impl IntoResponse {
    let rlist = SysWhiteJwtModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}
pub async fn delete(VJson(arg): VJson<WhiteJwtDel>) -> impl IntoResponse {
    let rid = SysWhiteJwtModel::del(arg).await;
    if let Ok(r) = rid.clone() {
        let cache = CacheManager::instance().await;
        let _ = cache.remove(&format!("user:{}", r)).await;
    }
    ApiResponse::from_result(rid)
}
pub async fn get_token(token_id: i64) -> Result<WhiteJwtRes> {
    let cache = CacheManager::instance().await;
    let white_jet = cache
        .get_value::<WhiteJwtRes>(&format!("user:{}", token_id))
        .await;
    match white_jet {
        Ok(r) => Ok(r),
        Err(_) => {
            let rmodel = SysWhiteJwtModel::get_token(token_id).await;
            if let Ok(r) = rmodel {
                let mut r = r;
                r.useronline = true;
                let _ = cache
                    .set_value_ex(&format!("user:{}", token_id), &r, 300)
                    .await;
                return Ok(r);
            }
            Err("None".into())
        }
    }
} 
pub async fn clear_user_info() -> Result<String> {
    SysWhiteJwtModel::clear_expire_user().await
}
