
use crate::service::prelude::*;
use crate::model::sys::model::msys_dict_type::{ 
    SysDictTypeModel,
    DictTypeAdd,DictTypeEdit,DictDataSearch,DictTypeDel
};

pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<DictDataSearch>,
) -> impl IntoResponse {
    let rlist = SysDictTypeModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}

pub async fn add(VJson(arg): VJson<DictTypeAdd>) -> impl IntoResponse {
    let r = SysDictTypeModel::add(arg).await;
    ApiResponse::from_result(r)
}

pub async fn edit(VJson(arg): VJson<DictTypeEdit>) -> impl IntoResponse {
     let r = SysDictTypeModel::edit(arg).await;
     ApiResponse::from_result(r)
}

pub async fn delete(VQuery(arg):VQuery<DictTypeDel>) -> impl IntoResponse {
    let r = SysDictTypeModel::del(arg).await;
    ApiResponse::from_result(r)
}