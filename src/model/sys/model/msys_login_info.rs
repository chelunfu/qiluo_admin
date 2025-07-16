pub use super::args::asys_login_info::*;
pub use super::entity::sys_login_info::{self, ActiveModel, Model as SysLoginInfoModel};
use crate::model::prelude::*;

impl SysLoginInfoModel {
    pub async fn list(arg: PageParams, search: LoginInfoSearch) -> Result<ListData<LoginInfoRes>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = sys_login_info::Entity::find();
        if let Some(user_name) = search.user_name {
            rmodel = rmodel.filter(sys_login_info::Column::UserName.contains(user_name));
        }

        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .order_by_desc(sys_login_info::Column::LoginTime)
            .into_model::<LoginInfoRes>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        let res = ListData {
            list,
            total,
            total_pages,
            page_num,
        };
        Ok(res)
    }

    pub async fn add(arg: LoginInfoAdd) -> Result<SysLoginInfoModel> {
        let db = DB().await;
        let id = GID().await;
        let amodel = sys_login_info::ActiveModel {
            info_id: Set(id),
            user_name: Set(arg.user_name),
            uid: Set(arg.uid),
            device_type: Set(arg.device_type),
            ipaddr: Set(arg.ipaddr),
            login_location: Set(arg.login_location),
            browser: Set(arg.browser),
            os: Set(arg.os),
            status: Set(arg.status),
            net_work: Set(arg.net_work),
            msg: Set(arg.msg),
            login_time: Set(arg.login_time),
        };
        amodel.insert(db).await.map_err(Into::into)
    }

    pub async fn edit(arg: LoginInfoEdit) -> Result<String> {
        let db = DB().await;
        let rmodel = sys_login_info::Entity::find()
            .filter(sys_login_info::Column::InfoId.eq(arg.info_id))
            .one(db)
            .await?;
        let model = if let Some(model) = rmodel {
            model
        } else {
            return Err("None Found".into());
        };
        let mut amodel: sys_login_info::ActiveModel = model.into();
        if arg.device_type.is_some() {
            amodel.device_type = Set(arg.device_type);
        }
        if arg.ipaddr.is_some() {
            amodel.ipaddr = Set(arg.ipaddr);
        }
        if arg.login_location.is_some() {
            amodel.login_location = Set(arg.login_location);
        }
        if arg.browser.is_some() {
            amodel.browser = Set(arg.browser);
        }
        if arg.os.is_some() {
            amodel.os = Set(arg.os);
        }
        if arg.status.is_some() {
            amodel.status = Set(arg.status);
        }
        if arg.msg.is_some() {
            amodel.msg = Set(arg.msg);
        }
        if arg.login_time.is_some() {
            amodel.login_time = Set(arg.login_time);
        }
        if arg.net_work.is_some() {
            amodel.net_work = Set(arg.net_work);
        }
        amodel.update(db).await?;
        Ok("Success".to_string())
    }
    pub async fn del() {}
}
