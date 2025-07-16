
use crate::service::prelude::*;
use crate::model::sys::model::msys_dict_data::{ 
    SysDictDataModel,
    DictDataAdd,DictDataSearch,DictDataEdit
    ,DictDataDel,DictDataType
};

pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<DictDataSearch>,
) -> impl IntoResponse {
    let rlist = SysDictDataModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}

pub async fn add(VJson(arg): VJson<DictDataAdd>) -> impl IntoResponse {
    let r = SysDictDataModel::add(arg).await;
    ApiResponse::from_result(r)
}

pub async fn edit(VJson(arg): VJson<DictDataEdit>) -> impl IntoResponse {
    let r = SysDictDataModel::edit(arg).await;
    ApiResponse::from_result(r)
}

pub async fn delete(VQuery(arg):VQuery<DictDataDel>) -> impl IntoResponse {
    let r = SysDictDataModel::del(arg).await;
    ApiResponse::from_result(r)
}

pub async fn get_by_type(VQuery(arg): VQuery<DictDataType>) -> impl IntoResponse {
    let r = SysDictDataModel::get_by_type(arg).await;
    ApiResponse::from_result(r)
}