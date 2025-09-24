use super::web_path::{WebPath, WebPathType};
use crate::service::sys::*;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
pub fn router_sys() -> WebPath {
    WebPath::new().nest(
        "/sys",
        WebPath::new()
            .nest("/user", sys_user())
            .nest("/menu", menu())
            .nest("/dept", sys_dept())
            .nest("/role", sys_role())
            .nest("/roleapi", sys_role_api())
            .nest("/logininfo", sys_login_info())
            .nest("/useronline", sys_user_online())
            .nest("/dictdata", sys_dict_data())
            .nest("/dicttype", sys_dict_type())
            .nest("/serverinfo", sys_server_info())
            .nest("/job", sys_job())
            .nest("/jobinfo", sys_job_info())
            .nest("/apipermission", sys_api_permission())
            .nest("/dashboard", sys_dashboard())
            .nest("/setting", sys_setting())
            .nest("/operationlog", sys_operation_log())
            .nest("/cache", sys_cache()),
    )
}

pub fn white_sys() -> Router {
    Router::new().nest(
        "/sys",
        Router::new()
            .nest("/test", sys_test())
            .nest("/auth", auth()),
    )
}

fn sys_test() -> Router {
    Router::new().route("/test1", get(s_sys_test::test)) 
}

fn auth() -> Router {
    Router::new()
        .route("/login", post(s_sys_user::login))
        .route("/get_captcha", get(s_sys_captcha::get_captcha))
}

fn sys_server_info() -> WebPath {
    WebPath::new().route(
        "/server_update",
        WebPathType::Get,
        Some("更新服务器信息"),
        get(s_sys_server_info::server_event),
    )
}

fn menu() -> WebPath {
    WebPath::new()
        .route(
            "/all_router",
            WebPathType::Get,
            Some("全部路由"),
            get(s_sys_menu::all_router),
        )
        .route(
            "/list",
            WebPathType::Get,
            Some("菜单列表"),
            get(s_sys_menu::list),
        )
        .route(
            "/tree",
            WebPathType::Get,
            Some("获取菜单树"),
            get(s_sys_menu::tree),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑菜单"),
            put(s_sys_menu::edit),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("添加菜单"),
            post(s_sys_menu::add),
        )
        .route(
            "/del",
            WebPathType::Delete,
            Some("删除菜单"),
            delete(s_sys_menu::delete),
        )
}

fn sys_user() -> WebPath {
    WebPath::new() 
        .route(
            "/list",
            WebPathType::Get,
            Some("获取用户列表"),
            get(s_sys_user::list),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("添加用户"),
            post(s_sys_user::add),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑用户"),
            put(s_sys_user::edit),
        )
        .route(
            "/depts_roles",
            WebPathType::Get,
            Some("获取用户拥有的用户部门和用户角色"),
            get(s_sys_user::depts_roles),
        )
        .route(
            "/reset_password",
            WebPathType::Put,
            Some("重置密码"),
            put(s_sys_user::reset_password),
        )
        .route(
            "/change_password",
            WebPathType::Put,
            Some("修改密码"),
            put(s_sys_user::change_password),
        )
        .route(
            "/delusers",
            WebPathType::Delete,
            Some("删除用户"),
            delete(s_sys_user::delete_users),
        )
        .route(
            "/update_avatar",
            WebPathType::Put,
            Some("更新头像"),
            put(s_sys_user::update_avatar),
        )
        .route(
            "/refersh_token",
            WebPathType::Put,
            Some("刷新token"),
            put(s_sys_user::refersh_token),
        )
        .route(
            "/login_out",
            WebPathType::Get,
            Some("用户退出"),
            get(s_sys_user::login_out),
        )
        .route(
            "/update_role_or_dept",
            WebPathType::Post,
            Some("更新用户拥有的用户部门和用户角色"),
            post(s_sys_user::update_role_or_dept),
        )
}
fn sys_dept() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取部门列表"),
            get(s_sys_dept::list_tree),
        )
        .route(
            "/dept_tree",
            WebPathType::Get,
            Some("获取部门树"),
            get(s_sys_dept::dept_tree),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑部门"),
            put(s_sys_dept::edit),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("添加部门"),
            post(s_sys_dept::add),
        )
        .route(
            "/del",
            WebPathType::Delete,
            Some("删除部门"),
            delete(s_sys_dept::delete),
        )
        .route(
            "/user_dept_name_list",
            WebPathType::Get,
            Some("获取用户拥有的部门名称列表"),
            get(s_sys_user_dept::user_dept_name_list),
        )
}

fn sys_role() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取角色列表"),
            get(s_sys_role::list),
        )
        .route(
            "/menu",
            WebPathType::Get,
            Some("获取角色菜单"),
            get(s_sys_role::menu),
        )
        .route(
            "/get_role_menus",
            WebPathType::Get,
            Some("获取角色菜单"),
            get(s_sys_role::get_role_menus),
        )
        .route(
            "/get_role_depts",
            WebPathType::Get,
            Some("获取角色部门"),
            get(s_sys_role::get_role_depts),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑角色"),
            put(s_sys_role::edit),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("添加角色"),
            post(s_sys_role::add),
        )
        .route(
            "/tree",
            WebPathType::Get,
            Some("获取角色树"),
            get(s_sys_role::tree),
        )
        .route(
            "/del",
            WebPathType::Delete,
            Some("删除角色"),
            delete(s_sys_role::delete),
        )
        .route(
            "/user_role_name_list",
            WebPathType::Get,
            Some("获取用户拥有的角色名称列表"),
            get(s_sys_user_role::user_role_name_list),
        )
}

fn sys_role_api() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取角色api列表"),
            get(s_sys_role_api::list),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("添加角色api"),
            post(s_sys_role_api::add),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑角色api"),
            put(s_sys_role_api::edit),
        )
        .route(
            "/del",
            WebPathType::Delete,
            Some("删除角色api"),
            delete(s_sys_role_api::delete),
        )
        .route(
            "/role_api_transfer_list",
            WebPathType::Get,
            Some("获取角色api所有的选择列表"),
            get(s_sys_role_api::role_api_transfer_list),
        )
        .route(
            "/add_many_role_api_transfer",
            WebPathType::Get,
            Some("添加多个角色api"),
            post(s_sys_role_api::add_many_role_api_transfer),
        )
        .route(
            "/role_permission_list",
            WebPathType::Get,
            Some("根据角色id获取api权限列表"),
            get(s_sys_role_api::role_permission_list),
        )
}

fn sys_login_info() -> WebPath {
    WebPath::new().route(
        "/list",
        WebPathType::Get,
        Some("获取登录日志列表"),
        get(s_sys_login_info::list),
    )
}
fn sys_user_online() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取在线用户列表"),
            get(s_sys_white_jwt::list),
        )
        .route(
            "/del",
            WebPathType::Delete,
            Some("退出在线用户"),
            delete(s_sys_white_jwt::delete),
        )
}

fn sys_dict_data() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取字典数据列表"),
            get(s_sys_dict_data::list),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("添加字典数据"),
            post(s_sys_dict_data::add),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑字典数据"),
            put(s_sys_dict_data::edit),
        )
        .route(
            "/del",
            WebPathType::Delete,
            Some("删除字典数据"),
            delete(s_sys_dict_data::delete),
        )
        .route(
            "/get_by_type",
            WebPathType::Get,
            Some("根据类型获取字典数据"),
            get(s_sys_dict_data::get_by_type),
        )
}

fn sys_dict_type() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取字典类型列表"),
            get(s_sys_dict_type::list),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("添加字典类型"),
            post(s_sys_dict_type::add),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑字典类型"),
            put(s_sys_dict_type::edit),
        )
        .route(
            "/del",
            WebPathType::Delete,
            Some("删除字典类型"),
            delete(s_sys_dict_type::delete),
        )
}

fn sys_job() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取定时任务列表"),
            get(s_sys_job::list),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("添加定时任务"),
            post(s_sys_job::add),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑定时任务"),
            put(s_sys_job::edit),
        )
        .route(
            "/del",
            WebPathType::Delete,
            Some("删除定时任务"),
            delete(s_sys_job::delete),
        )
        .route(
            "/validate_cron",
            WebPathType::Post,
            Some("验证cron表达式"),
            post(s_sys_job::validate_cron),
        )
        .route(
            "/hand_execute_job",
            WebPathType::Post,
            Some("执行定时任务"),
            post(s_sys_job::hand_execute_job),
        )
}

fn sys_job_info() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取定时任务日志列表"),
            get(s_sys_job_log::list),
        )
        .route(
            "/add",
            WebPathType::Post,
            Some("添加定时任务日志"),
            post(s_sys_job_log::add),
        )
}

fn sys_api_permission() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取api权限列表"),
            get(s_sys_api_permission::list),
        )
        .route(
            "/edit",
            WebPathType::Put,
            Some("编辑api权限"),
            put(s_sys_api_permission::edit),
        )
}

fn sys_dashboard() -> WebPath {
    WebPath::new()
        .nest(
            "/analysis",
            WebPath::new()
                .route(
                    "/total",
                    WebPathType::Get,
                    Some("获取仪表盘数据"),
                    get(s_sys_dashboard::analysis_total),
                )
                .route(
                    "/userAccessSource",
                    WebPathType::Get,
                    Some("获取仪表盘数据"),
                    get(s_sys_dashboard::user_access_source),
                )
                .route(
                    "/weeklyUserActivity",
                    WebPathType::Get,
                    Some("获取仪表盘数据"),
                    get(s_sys_dashboard::weekly_user_activity),
                )
                .route(
                    "/monthlySales",
                    WebPathType::Get,
                    Some("获取仪表盘数据"),
                    get(s_sys_dashboard::monthly_sales),
                ),
        )
        .nest(
            "/workplace",
            WebPath::new()
                .route(
                    "/total",
                    WebPathType::Get,
                    Some("获取仪表盘数据"),
                    get(s_sys_dashboard::workplace_total),
                )
                .route(
                    "/project",
                    WebPathType::Get,
                    Some("获取仪表盘数据"),
                    get(s_sys_dashboard::workplace_project),
                )
                .route(
                    "/dynamic",
                    WebPathType::Get,
                    Some("获取仪表盘数据"),
                    get(s_sys_dashboard::workplace_dynamic),
                )
                .route(
                    "/team",
                    WebPathType::Get,
                    Some("获取仪表盘数据"),
                    get(s_sys_dashboard::workplace_team),
                )
                .route(
                    "/radar",
                    WebPathType::Get,
                    Some("获取仪表盘数据"),
                    get(s_sys_dashboard::workplace_radar),
                ),
        )
}

fn sys_setting() -> WebPath {
    WebPath::new()
}

fn sys_operation_log() -> WebPath {
    WebPath::new().route(
        "/list",
        WebPathType::Get,
        Some("获取操作日志列表"),
        get(s_sys_operation_log::list),
    )
}

fn sys_cache() -> WebPath {
    WebPath::new()
        .route(
            "/list",
            WebPathType::Get,
            Some("获取缓存列表"),
            get(s_sys_cache::list),
        )
        .route(
            "/clear",
            WebPathType::Post,
            Some("清空缓存"),
            post(s_sys_cache::clear),
        )
}
