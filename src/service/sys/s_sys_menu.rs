use crate::model::sys::model::msys_menu::{
    ListMeta, MenuAdd, MenuDel, MenuEdit, MenuResp, RouteMeta, SysMenuListTree, SysMenuModel,
    SysMenuRouterTree as SysMenuRouteTree, UserMenu,
};
use crate::service::prelude::*;

pub async fn list(uinfo: UserInfo) -> impl IntoResponse {
    let menus: Vec<MenuResp> = SysMenuModel::get_menus(false, false, true, uinfo.rid)
        .await
        .unwrap();
    let menu_data = self::get_menu_list_data(menus);
    let menu_tree = self::get_menu_list_tree(menu_data, 0);
    ApiResponse::ok(menu_tree)
}

pub async fn edit(VJson(arg): VJson<MenuEdit>) -> impl IntoResponse {
    let umodel = SysMenuModel::edit(arg).await;
    ApiResponse::from_result(umodel)
}
pub async fn add(VJson(arg): VJson<MenuAdd>) -> impl IntoResponse {
    let amodel = SysMenuModel::add(arg).await;
    ApiResponse::from_result(amodel)
}
pub async fn delete(VQuery(arg): VQuery<MenuDel>) -> impl IntoResponse {
    let dmodel = SysMenuModel::del(arg).await;
    ApiResponse::from_result(dmodel)
}
pub async fn tree(uinfo: UserInfo) -> impl IntoResponse {
    let menus = SysMenuModel::get_menus(true, false, true, uinfo.rid)
        .await
        .unwrap();
    let menu_data = self::get_menu_list_data(menus);
    let menu_tree = self::get_menu_list_tree(menu_data, 0);
    ApiResponse::ok(menu_tree)
}

pub async fn all_router(uinfo: UserInfo) -> impl IntoResponse {
    let menus: Vec<MenuResp> = SysMenuModel::get_menus(true, false, true, uinfo.rid)
        .await
        .unwrap();
    info!("{:?}", menus.len());
    let menu_data = self::get_menu_route_data(menus);

    let menu_tree = self::get_menu_route_tree(menu_data, 0);

    ApiResponse::ok(menu_tree)
}

fn get_menu_list_tree(user_menus: Vec<SysMenuListTree>, pid: i64) -> Vec<SysMenuListTree> {
    let mut menu_tree: Vec<SysMenuListTree> = Vec::new();
    for mut user_menu in user_menus.clone() {
        if user_menu.pid == pid {
            user_menu.children = Some(get_menu_list_tree(user_menus.clone(), user_menu.id));
            menu_tree.push(user_menu.clone());
        }
    }
    menu_tree
}
fn get_menu_list_data(menus: Vec<MenuResp>) -> Vec<SysMenuListTree> {
    let mut menu_res: Vec<SysMenuListTree> = Vec::new();
    for menu in menus {
        let meta = ListMeta {
            icon: menu.icon.clone(),
            title: menu.title.clone(),
            no_cache: menu.no_cache,
            hidden: menu.hidden,
            active_menu: menu.active_menu,
            breadcrumb: menu.breadcrumb,
            always_show: menu.always_show,
            affix: menu.affix,
            can_to: menu.can_to,
            no_tags_view: menu.no_tags_view,
            i18nkey: menu.i18nkey,
        };
        let menu_tree = SysMenuListTree {
            meta,
            id: menu.id,
            pid: menu.pid,
            path: menu.path,
            order: menu.order,
            redirect: menu.redirect,
            menu_type: menu.menu_type,
            status: menu.status,
            component: menu.component,
            name: menu.name,
            ..Default::default()
        };
        menu_res.push(menu_tree);
    }
    menu_res
}

fn get_menu_route_data(menus: Vec<MenuResp>) -> Vec<SysMenuRouteTree> {
    let mut menu_res: Vec<SysMenuRouteTree> = Vec::new();
    for menu in menus {
        let meta = RouteMeta {
            icon: menu.icon.clone(),
            title: menu.title.clone(),
            no_cache: menu.no_cache,
            href: menu.href,
            hidden: menu.hidden,
            active_menu: menu.active_menu,
            breadcrumb: menu.breadcrumb,
            always_show: menu.always_show,
            affix: menu.affix,
            can_to: menu.can_to,
            no_tags_view: menu.no_tags_view,
            i18nkey: menu.i18nkey,
        };
        let user_menu = UserMenu {
            meta,
            id: menu.id,
            pid: menu.pid,
            path: menu.path.clone(),
            title: menu.title.clone(),
            order: menu.order,
            redirect: menu.redirect,
            menu_type: menu.menu_type,
            status: menu.status,
            component: menu.component.clone(),
            name: menu.name,
        };
        let menu_tree = SysMenuRouteTree {
            user_menu,
            ..Default::default()
        };
        menu_res.push(menu_tree);
    }
    menu_res
}

fn get_menu_route_tree(user_menus: Vec<SysMenuRouteTree>, pid: i64) -> Vec<SysMenuRouteTree> {
    let mut menu_tree: Vec<SysMenuRouteTree> = Vec::new();
    for mut user_menu in user_menus.clone() {
        if user_menu.user_menu.pid == pid {
            user_menu.children = Some(get_menu_route_tree(
                user_menus.clone(),
                user_menu.user_menu.id,
            ));

            menu_tree.push(user_menu.clone());
        }
    }
    menu_tree
}
