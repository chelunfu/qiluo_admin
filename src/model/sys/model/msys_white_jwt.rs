pub use super::args::asys_white_jwt::*;
pub use super::entity::{
    sys_login_info,
    sys_white_jwt::{self, ActiveModel, Model as SysWhiteJwtModel},
};
use crate::model::prelude::*;

impl SysWhiteJwtModel {
    pub async fn clear_expire_user() -> Result<String> {
        let now = Local::now().naive_local();
        let time = now.and_utc().timestamp();
        let db = DB().await;
        let mut rmodel = sys_white_jwt::Entity::update_many();
        rmodel = rmodel.col_expr(sys_white_jwt::Column::DeletedAt, Expr::value(now));
        rmodel = rmodel.filter(sys_white_jwt::Column::DeletedAt.is_null());
        rmodel = rmodel.filter(sys_white_jwt::Column::TokenExpr.lt(time));
        let r = rmodel.exec(db).await?;
        Ok(format!("Cleaned up {} expired users", r.rows_affected))
    }

    pub async fn list(arg: PageParams, _: WhiteJwtSearch) -> Result<ListData<WhiteJwtRes>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db: &DatabaseConnection = DB().await;
        let mut rmodel = sys_white_jwt::Entity::find();

        rmodel = rmodel.filter(sys_white_jwt::Column::DeletedAt.is_null());
        rmodel = rmodel.join_rev(
            JoinType::LeftJoin,
            sys_login_info::Entity::belongs_to(sys_white_jwt::Entity)
                .from(sys_login_info::Column::InfoId)
                .to(sys_white_jwt::Column::InfoId)
                .into(),
        );
        rmodel = rmodel.columns([
            sys_login_info::Column::UserName,
            sys_login_info::Column::DeviceType,
            sys_login_info::Column::Ipaddr,
            sys_login_info::Column::LoginLocation,
            sys_login_info::Column::NetWork,
            sys_login_info::Column::Browser,
            sys_login_info::Column::Os,
            sys_login_info::Column::Msg,
            sys_login_info::Column::LoginTime,
        ]);
        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .order_by_desc(sys_white_jwt::Column::CreatedAt)
            .into_model::<WhiteJwtRes>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let mut list = paginator.fetch_page(page_num - 1).await?;
        let cache = CacheManager::instance().await;
        for item in list.iter_mut() {
            item.useronline = cache.contains_key(&format!("user:{}", item.token_id)).await;
        }
        let res = ListData {
            list,
            total,
            total_pages,
            page_num,
        };
        Ok(res)
    }
    pub async fn add(arg: WhiteJwtAdd) -> Result<SysWhiteJwtModel> {
        let db = DB().await;
        let id = GID().await;
        let now = Local::now().naive_local();
        let amodel = sys_white_jwt::ActiveModel {
            jid: Set(id),
            uid: Set(arg.uid),
            token_id: Set(arg.token_id),
            token_expr: Set(arg.token_expr),
            info_id: Set(arg.info_id),
            created_at: Set(now),
            update_at: Set(now),
            deleted_at: Set(None),
        };
        amodel.insert(db).await.map_err(Into::into)
    }
    pub async fn del(arg: WhiteJwtDel) -> Result<i64> {
        let db = DB().await;
        let model = sys_white_jwt::Entity::find_by_id(arg.jid)
            .one(db)
            .await?
            .ok_or("Record not found")?;

        let now = Local::now().naive_local();
        let mut amodel: sys_white_jwt::ActiveModel = model.into();
        amodel.update_at = Set(now);
        amodel.deleted_at = Set(Some(now));
        amodel
            .update(db)
            .await
            .map(|m| m.token_id)
            .map_err(Into::into)
    }

    pub async fn edit(arg: WhiteJwtEdit) -> Result<String> {
        let db = DB().await;
        sys_white_jwt::Entity::update_many()
            .col_expr(
                sys_white_jwt::Column::TokenExpr,
                Expr::value(arg.token_expr),
            )
            .filter(sys_white_jwt::Column::TokenId.eq(arg.token_id))
            .exec(db)
            .await?;
        Ok("Success".to_string())
    }

    ///检查ID是否存在
    pub async fn get_token(token_id: i64) -> Result<WhiteJwtRes> {
        let db: &DatabaseConnection = DB().await;
        let mut rmodel = sys_white_jwt::Entity::find();
        rmodel = rmodel.filter(sys_white_jwt::Column::DeletedAt.is_null());
        rmodel = rmodel.join_rev(
            JoinType::LeftJoin,
            sys_login_info::Entity::belongs_to(sys_white_jwt::Entity)
                .from(sys_login_info::Column::InfoId)
                .to(sys_white_jwt::Column::InfoId)
                .into(),
        );
        rmodel = rmodel.columns([
            sys_login_info::Column::UserName,
            sys_login_info::Column::DeviceType,
            sys_login_info::Column::Ipaddr,
            sys_login_info::Column::LoginLocation,
            sys_login_info::Column::NetWork,
            sys_login_info::Column::Browser,
            sys_login_info::Column::Os,
            sys_login_info::Column::Status,
            sys_login_info::Column::Msg,
            sys_login_info::Column::LoginTime,
        ]);
        rmodel
            .filter(sys_white_jwt::Column::TokenId.eq(token_id))
            .into_model::<WhiteJwtRes>()
            .one(db)
            .await?
            .ok_or("Record not found".into())
    }
}
