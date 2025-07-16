pub use super::args::asys_menu::*;
pub use super::entity::sys_menu::{self, ActiveModel, Model as SysMenuModel};
use super::entity::sys_role_menu;
use crate::model::prelude::*;

impl SysMenuModel {
    pub async fn list(arg: PageParams) -> Result<ListData<MenuResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = sys_menu::Entity::find();

        rmodel = rmodel.filter(sys_menu::Column::DeletedAt.is_null());

        let total = rmodel.clone().count(db).await.unwrap();
        let paginator = rmodel
            .order_by_asc(sys_menu::Column::Order)
            .into_model::<MenuResp>()
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

    pub async fn add(arg: MenuAdd) -> Result<String> {
        let db = DB().await;
        let rmodel = sys_menu::Entity::find()
            .filter(
                Condition::all()
                    .add(sys_menu::Column::Pid.eq(arg.pid))
                    .add(sys_menu::Column::Path.eq(arg.path.clone())),
            )
            .one(db)
            .await?;
        if rmodel.is_some() {
            return Err("Add Failed".into());
        }

        let id = GID().await;
        let now = Local::now().naive_local();
        let amodel: ActiveModel = sys_menu::ActiveModel {
            id: Set(id),
            name: Set(arg.name),
            title: Set(arg.meta.title),
            i18nkey: Set(arg.meta.i18nkey),
            pid: Set(arg.pid),
            order: Set(arg.order),
            path: Set(arg.path),
            component: Set(arg.component),
            redirect: Set(arg.redirect),
            no_cache: Set(arg.meta.no_cache),
            menu_type: Set(arg.menu_type),
            hidden: Set(arg.meta.hidden),
            status: Set(arg.status),
            perms: Set(None),
            icon: Set(arg.meta.icon),
            breadcrumb: Set(arg.meta.breadcrumb),
            active_menu: Set(arg.meta.active_menu),
            no_tags_view: Set(arg.meta.no_tags_view),
            can_to: Set(arg.meta.can_to),
            affix: Set(arg.meta.affix),
            always_show: Set(arg.meta.always_show),
            remark: Set(arg.remark),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
            ..Default::default()
        };
        amodel.insert(db).await?;
        Ok("Success".to_string())
    }

    pub async fn edit(arg: MenuEdit) -> Result<String> {
        info!("Edit Menu:{:?}", arg.clone());
        let db = DB().await;

        let rmodel = sys_menu::Entity::find_by_id(arg.id).one(db).await?;
        let model = if let Some(model) = rmodel {
            model
        } else {
            return Err("Model Failed".into());
        };
        let now = Local::now().naive_local();
        let mut amodel: sys_menu::ActiveModel = model.into();
        amodel.pid = Set(arg.pid);
        amodel.name = Set(arg.name);
        amodel.title = Set(arg.meta.title);
        amodel.menu_type = Set(arg.menu_type);
        amodel.path = Set(arg.path);
        amodel.component = Set(arg.component);
        amodel.status = Set(arg.status);
        amodel.order = Set(arg.order);
        amodel.icon = Set(arg.meta.icon);
        amodel.hidden = Set(arg.meta.hidden);
        amodel.no_cache = Set(arg.meta.no_cache);
        amodel.redirect = Set(arg.redirect);
        amodel.i18nkey = Set(arg.meta.i18nkey);
        amodel.breadcrumb = Set(arg.meta.breadcrumb);
        amodel.active_menu = Set(arg.meta.active_menu);
        amodel.no_tags_view = Set(arg.meta.no_tags_view);
        amodel.can_to = Set(arg.meta.can_to);
        amodel.remark = Set(arg.remark);
        amodel.updated_at = Set(Some(now));
        amodel.update(db).await?;
        Ok("Success".to_string())
    }

    pub async fn del(arg: MenuDel) -> Result<String> {
        let db = DB().await;
        let rmodel = sys_menu::Entity::find()
            .filter(sys_menu::Column::Pid.eq(arg.id))
            .count(db)
            .await?;
        if rmodel > 0 {
            return Err("Failed,There are sub-menu".into());
        }

        sys_menu::Entity::delete_by_id(arg.id).exec(db).await?;
        Ok("Success".to_string())
    }

    pub async fn get_menus(
        is_router: bool,
        is_only_api: bool,
        is_only_enabled: bool,
        role_id: i64,
    ) -> Result<Vec<MenuResp>> {
        let db = DB().await;

        let mut rmodel = sys_menu::Entity::find();
        rmodel = rmodel.filter(sys_menu::Column::DeletedAt.is_null());
        if is_router {
            rmodel = rmodel.filter(sys_menu::Column::MenuType.ne("F"));
        };
        if is_only_api {
            rmodel = rmodel.filter(sys_menu::Column::MenuType.eq("F"));
        };
        if is_only_enabled {
            rmodel = rmodel.filter(sys_menu::Column::Status.eq(0));
        };
        if !APPCOFIG.system.super_role.contains(&role_id) {
            rmodel = rmodel.join_rev(
                JoinType::LeftJoin,
                sys_role_menu::Entity::belongs_to(sys_menu::Entity)
                    .from(sys_role_menu::Column::MenuId)
                    .to(sys_menu::Column::Id)
                    .into(),
            );
            rmodel = rmodel.filter(sys_role_menu::Column::RoleId.eq(role_id));
        }
        let res = rmodel
            .order_by(sys_menu::Column::Order, Order::Asc)
            .into_model::<MenuResp>()
            .all(db)
            .await?;
        Ok(res)
    }
}
