use crate::model::prelude::*;



#[derive(Serialize, Clone, Debug, Default, Deserialize)]
pub struct SysMenuListTree {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub pid: i64,
    pub path: Option<String>,
    pub order: i32,
    pub redirect: Option<String>,
    pub name:  Option<String>, 
    pub menu_type: String,
    pub component: Option<String>,
    pub status: String, 
    
    #[serde(flatten)]
    pub meta: ListMeta,

    pub children: Option<Vec<SysMenuListTree>>,
}

#[derive(Serialize, Clone, Debug, Default, Deserialize)]
pub struct ListMeta {   
    pub title: String,
    pub icon: Option<String>,
    #[serde(rename = "noCache", with = "string_to_bool")]
    pub no_cache: String,
    #[serde(with = "string_to_bool")]
    pub breadcrumb: String,
    #[serde(with = "string_to_bool")]
    pub affix: String,
    #[serde(rename = "noTagsView", with = "string_to_bool")]
    pub no_tags_view: String,
    #[serde(with = "string_to_bool")]
    pub hidden: String,
    #[serde(rename = "activeMenu", with = "string_to_bool")]
    pub active_menu: String,
    #[serde(rename = "canTo", with = "string_to_bool")]
    pub can_to: String,
    #[serde(rename = "alwaysShow", with = "string_to_bool")]
    pub always_show: String,
    pub i18nkey: Option<String>
}


#[derive(Deserialize, Clone, Debug, Default, Validate)]
pub struct MenuSearch {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    pub path: Option<String>,
    pub redirect: Option<String>,
    pub name:  Option<String>,
    pub title: String,
    pub menu_type: String,
    pub component: Option<String>,
    pub status: String
}

//编辑
#[derive(Deserialize, Clone, Debug, Default, Validate)]
pub struct MenuEdit {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub pid: i64,
    pub path: Option<String>,
    pub redirect: Option<String>,
    pub name:  Option<String>,
    pub menu_type: String,
    pub component: Option<String>,
    pub status: String, 
    pub order: i32,
    pub remark: Option<String>,
    #[serde(flatten)]
    pub meta: ListMeta,
}

#[derive(Debug, Deserialize, Serialize, FromQueryResult, Clone)]
pub struct MenuResp {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    pub name:  Option<String>,
    pub title: String,
    pub i18nkey: Option<String>,
    #[serde(with = "i64_to_string")]
    pub pid: i64,
    pub order: i32,
    pub path: Option<String>,
    pub component: Option<String>,
    pub redirect: Option<String>, 
    pub href: Option<String>,
    pub no_cache: String,
    pub menu_type: String,
    pub hidden: String,
    pub active_menu: String,
    pub always_show: String,
    pub breadcrumb: String,
    pub affix: String,
    pub no_tags_view: String,
    pub can_to: String,
    pub status: String,
    pub perms: Option<String>,
    pub icon: Option<String>,
    pub remark: Option<String>,
}

#[derive(Serialize, Clone, Debug, Default, Deserialize)]
pub struct UserMenu {
    #[serde(with = "i64_to_string")]
    pub id: i64,
    #[serde(with = "i64_to_string")]
    pub pid: i64,
    pub path: Option<String>,
    pub order: i32,
    pub redirect: Option<String>,
    pub name:  Option<String>,
    pub title: String,
    pub menu_type: String,
    pub component: Option<String>,
    pub status: String,
    pub meta: RouteMeta,
}
#[derive(Serialize, Clone, Debug, Default, Deserialize)]
pub struct SysMenuRouterTree {
    #[serde(flatten)]
    pub user_menu: UserMenu,
    pub children: Option<Vec<SysMenuRouterTree>>,
}
#[derive(Serialize, Clone, Debug, Default, Deserialize)]
pub struct RouteMeta {
    pub href: Option<String>,
    #[serde(rename = "alwaysShow", with = "string_to_bool")]
    pub always_show: String,
    pub title: String,
    pub icon: Option<String>,
    #[serde(rename = "noCache", with = "string_to_bool")]
    pub no_cache: String,
    #[serde(with = "string_to_bool")]
    pub breadcrumb: String,
    #[serde(with = "string_to_bool")]
    pub affix: String,
    #[serde(rename = "noTagsView", with = "string_to_bool")]
    pub no_tags_view: String,
    #[serde(with = "string_to_bool")]
    pub hidden: String,
    #[serde(rename = "activeMenu", with = "string_to_bool")]
    pub active_menu: String,
    #[serde(rename = "canTo", with = "string_to_bool")]
    pub can_to: String,
    pub i18nkey: Option<String>
}

#[derive(Deserialize, Clone, Debug, Default, Validate)]
pub struct MenuAdd {
    #[serde(with = "i64_to_string")]
    pub pid: i64,
    pub path: Option<String>,
    pub redirect: Option<String>,
    pub name:  Option<String>,
    pub menu_type: String,
    pub api:  Option<String>,
    pub order: i32,
    pub method:  Option<String>,
    pub component: Option<String>,
    pub status: String,
    pub remark: Option<String>,
    #[serde(flatten)]
    pub meta: ListMeta,
}
#[derive(Deserialize, Clone, Debug, Default, Validate)]
pub struct MenuDel {
    #[serde(with = "i64_to_string")]
    pub id: i64
}
