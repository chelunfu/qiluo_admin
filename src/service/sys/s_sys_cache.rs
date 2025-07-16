use crate::model::sys::args::acache::*;
use crate::service::prelude::*;
pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<CacheSearch>,
) -> impl IntoResponse {
    let page_num = arg.page_num.unwrap_or(1);
    let page_per_size = arg.page_size.unwrap_or(10);
    let cache = CacheManager::instance().await;
    let rlist = cache
        .get_all_paginated(page_num, page_per_size, search.key)
        .await;
     
    ApiResponse::from_result(rlist)
}

pub async fn clear() -> impl IntoResponse {}
