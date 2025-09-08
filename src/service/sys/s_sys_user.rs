use crate::{service::prelude::*, worker::AppWorker};

use super::{s_sys_login_info::login_info_function, s_sys_upload};
use crate::model::sys::args::acaptch::CaptchaCacheInfo;
use crate::worker::logininfo::LoginInfoWorker;

use headers::HeaderMap;

use model::sys::model::{
    msys_login_info::{LoginInfoAdd, LoginInfoMsg, SysLoginInfoModel},
    msys_user::{
        ChangePasswordParams, DeptAndRole, DeptsAndRoles, LoginParams, ResetPasswordParams,
        SysUserAdd, SysUserEdit, SysUserModel, UserAvatarEdit, UserId, UserIds, UserInfoDetail,
        UserInfoRes, UserSearch,
    },
    msys_user_dept::SysUserDeptModel,
    msys_user_role::SysUserRoleModel,
    msys_white_jwt::{SysWhiteJwtModel, WhiteJwtAdd, WhiteJwtEdit},
};

pub async fn get_user_info(uid: i64) -> Result<UserInfoDetail> {
    let ruser = SysUserModel::find_by_id(uid).await;
    let user = match ruser {
        Some(user) => user,
        None => {
            return Err(Error::Message("User does not exist".into()));
        }
    };
    let userinfo = UserInfoDetail {
        username: user.user_name,
        nickname: user.nick_name,
        email: user.email,
        phone: user.phonenumber,
        did: user.dept_id,
        rid: user.role_id,
    };
    Ok(userinfo)
}

pub async fn login(header: HeaderMap, VJson(arg): VJson<LoginParams>) -> impl IntoResponse {
    let capid = arg.captchaid;
    let cache = CacheManager::instance().await;
    let captcha_info = if let Ok(info) = cache
        .get_oneuse_value::<CaptchaCacheInfo>(&format!("capcha:{}", capid))
        .await
    {
        info
    } else {
        return ApiResponse::bad_request("Invalid or expired verification code");
    };
    info!("captcha:{}  {}", captcha_info.client_id, arg.client_id);
    if arg.client_id != captcha_info.client_id {
        return ApiResponse::bad_request("Verification code error");
    }

    info!("header:{:?}", header.clone());
    let ipaddr = get_remote_ip(header.clone());
    info!("ip:{}", ipaddr);
    let user_agent_str = header.get("user-agent").unwrap().to_str().unwrap();
    let useragent = login_info_function::get_user_agent_info(user_agent_str);

    let mut login_add = LoginInfoAdd {
        user_name: arg.username.clone(),
        browser: Some(useragent.browser),
        os: Some(useragent.os),
        ipaddr: Some(ipaddr.clone()),
        login_time: Some(Local::now().naive_local()),
        login_location: Some("Unknown City".into()),
        net_work: Some("Unknown Network".into()),
        device_type: Some(useragent.device),
        ..Default::default()
    };
    if arg.captcha.to_lowercase() != captcha_info.cache_text.to_lowercase() {
        login_add.status = Some("1".into());
        login_add.msg = Some(format!(
            "Entered code: {}, verification code incorrect",
            arg.captcha
        ));
        let rlogin_info_add = SysLoginInfoModel::add(login_add).await;
        if let Ok(login_info_add) = rlogin_info_add {
            let info_msg = LoginInfoMsg {
                ipaddr,
                info_id: login_info_add.info_id,
            };
            let _ = LoginInfoWorker::execute_async(info_msg).await;
        }

        return ApiResponse::bad_request("Verification code error");
    }

    let ouser = if let Some(email) = arg.email {
        SysUserModel::find_by_email(email.as_str()).await
    } else {
        SysUserModel::find_by_username(arg.username.as_str()).await
    };

    let user = match ouser {
        Some(user) => user,
        None => {
            login_add.status = Some("1".into());
            login_add.msg = Some("User does not exist".into());
            let rlogin_info_add = SysLoginInfoModel::add(login_add).await;
            if let Ok(login_info_add) = rlogin_info_add {
                let info_msg = LoginInfoMsg {
                    ipaddr,
                    info_id: login_info_add.info_id,
                };
                let _ = LoginInfoWorker::execute_async(info_msg).await;
            }

            return ApiResponse::bad_request("User does not exist");
        }
    };

    if !util::verify_password(&arg.password, user.password.as_str()) {
        login_add.status = Some("1".into());
        login_add.msg = Some("Incorrect password".into());
        let rlogin_info_add = SysLoginInfoModel::add(login_add).await;
        if let Ok(login_info_add) = rlogin_info_add {
            let info_msg = LoginInfoMsg {
                ipaddr,
                info_id: login_info_add.info_id,
            };
            let _ = LoginInfoWorker::execute_async(info_msg).await;
        }
        return ApiResponse::bad_request("Incorrect password");
    }

    let authplay = AuthPayload {
        uid: user.id,
        did: user.dept_id,
        rid: user.role_id,
        username: user.user_name.clone(),
    };
    let token_id: i64 = GID().await;
    let token = jwt::authorize(authplay.clone(), token_id).await.unwrap();

    login_add.status = Some("0".into());
    login_add.msg = Some("Success".into());
    login_add.uid = user.id;
    let rlogin_info_add = SysLoginInfoModel::add(login_add).await;
    let login_info_add = match rlogin_info_add {
        Ok(model) => {
            let info_msg = LoginInfoMsg {
                ipaddr,
                info_id: model.info_id,
            };
            let _ = LoginInfoWorker::execute_async(info_msg).await;
            model
        }
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };

    let addjwt = WhiteJwtAdd {
        token_id,
        uid: authplay.uid,
        token_expr: token.exp,
        info_id: login_info_add.info_id,
    };
    let _ = match SysWhiteJwtModel::add(addjwt).await {
        Ok(model) => model,
        Err(e) => {
            return ApiResponse::bad_request(e.to_string());
        }
    };

    info!("header:{:?}", header);
    let serverconfig = APPCOFIG.server.clone();
    let userinfo = UserInfoRes {
        username: user.user_name,
        nickname: user.nick_name,
        email: user.email,
        phone: user.phonenumber,
        did: user.dept_id,
        rid: user.role_id,
        avatar: match user.avatar {
            Some(a) => Some(format!("{}/{}", serverconfig.domainname, a)),
            None => None,
        },
    };
    ApiResponse::ok(json!({ "token":token,"userinfo":userinfo}))
}

pub async fn refersh_token(userinfo: UserInfo) -> impl IntoResponse {
    let authplay = AuthPayload {
        uid: userinfo.uid,
        did: userinfo.did,
        rid: userinfo.rid,
        username: userinfo.username,
    };
    let token_id: i64 =userinfo.token_id;
    let token = jwt::authorize(authplay.clone(), token_id).await.unwrap();
    let white_edit = WhiteJwtEdit {
        token_expr: token.exp,
        token_id,
    };
    if let Err(e) = SysWhiteJwtModel::edit(white_edit).await {
        return ApiResponse::bad_request(e.to_string());
    }
    ApiResponse::ok(token)
}

pub async fn login_out(_: UserInfo) -> impl IntoResponse {
    ApiResponse::ok("ok")
}

//重置密码
pub async fn reset_password(
    user: UserInfo,
    VJson(arg): VJson<ResetPasswordParams>,
) -> impl IntoResponse {
    let user = SysUserModel::reset_password(user.uid, arg).await;
    info!("user:{:?}", user);
    ApiResponse::from_result(user)
}
pub async fn change_password(VJson(arg): VJson<ChangePasswordParams>) -> impl IntoResponse {
    let user = SysUserModel::chagne_password(arg).await;
    info!("user:{:?}", user);
    ApiResponse::from_result(user)
}
 

pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<UserSearch>,
) -> impl IntoResponse {
    let rlist = SysUserModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}

pub async fn edit(VJson(arg): VJson<SysUserEdit>) -> impl IntoResponse {
    let user = match SysUserModel::edit(arg.clone()).await {
        Ok(us) => us,
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };

    let user_role_result = SysUserRoleModel::add_user_roles(user.id, arg.role_ids.clone()).await;
    if user_role_result.is_err() {
        return ApiResponse::bad_request("Add Role Faild");
    }
    let user_dept_result = SysUserDeptModel::add_user_depts(user.id, arg.dept_ids).await;
    ApiResponse::from_result(user_dept_result)
}

pub async fn add(VJson(arg): VJson<SysUserAdd>) -> impl IntoResponse {
    let mut arg = arg.clone();
    arg.password = match util::hash_password(&arg.password) {
        Ok(ps) => ps,
        Err(_) => return ApiResponse::bad_request("Pass Error"),
    };
    let user = match SysUserModel::add(arg.clone()).await {
        Ok(us) => us,
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    let user_role_result = SysUserRoleModel::add_user_roles(user.id, arg.role_ids.clone()).await;
    if user_role_result.is_err() {
        return ApiResponse::bad_request("Add Role Faild");
    }
    let user_dept_result = SysUserDeptModel::add_user_depts(user.id, arg.dept_ids).await;
    ApiResponse::from_result(user_dept_result)
}
pub async fn delete_users(VJson(arg): VJson<UserIds>) -> impl IntoResponse {
    let r = SysUserModel::delete(arg.uids).await;
    ApiResponse::from_result(r)
}

pub async fn update_avatar(
    userinfo: UserInfo,
    VJson(arg): VJson<UserAvatarEdit>,
) -> impl IntoResponse {
    match s_sys_upload::save_base64_img(&arg.avatar).await {
        Ok((domain_avatar, avatar)) => {
            let _ = SysUserModel::update_avatar(userinfo.uid, avatar).await;
            ApiResponse::ok(domain_avatar)
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}

pub async fn update_role_or_dept(
    userinfo: UserInfo,
    VJson(arg): VJson<DeptAndRole>,
) -> impl IntoResponse {
    let r = SysUserModel::update_role_or_dept(userinfo.uid, arg).await;
    ApiResponse::from_result(r)
}

pub async fn depts_roles(VQuery(arg): VQuery<UserId>) -> impl IntoResponse {
    let uid = arg.uid;
    let depts = match SysUserDeptModel::user_dept_list(uid).await {
        Ok(depts) => depts,
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    let roles = match SysUserRoleModel::user_role_list(uid).await {
        Ok(roles) => roles,
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    let userdeptrole = DeptsAndRoles { depts, roles };
    ApiResponse::ok(userdeptrole)
}

pub fn get_remote_ip(header: HeaderMap) -> String {
    let ip: String = match header.get("X-Forwarded-For") {
        Some(x) => {
            let mut ips = x.to_str().unwrap().split(',');
            ips.next().unwrap().trim().to_string()
        }
        None => match header.get("X-Real-IP") {
            Some(x) => x.to_str().unwrap().to_string(),
            None => "Unknown IP".to_string(),
        },
    };
    ip
}
