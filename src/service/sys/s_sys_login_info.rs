use crate::model::sys::model::msys_login_info::{
    LoginInfoEdit, LoginInfoMsg, LoginInfoSearch, SysLoginInfoModel,
};
use crate::service::prelude::*;

pub async fn list(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<LoginInfoSearch>,
) -> impl IntoResponse {
    let rlist = SysLoginInfoModel::list(arg, search).await;
    ApiResponse::from_result(rlist)
}

pub async fn update_login_info(arg: LoginInfoMsg) {
    let client_info = login_info_function::get_city_by_ip(&arg.ipaddr).await;

    let logadd = LoginInfoEdit {
        info_id: arg.info_id,
        login_location: Some(client_info.location),
        ipaddr: Some(client_info.ip),
        net_work: Some(client_info.net_work),
        ..Default::default()
    };
    let _ = SysLoginInfoModel::edit(logadd).await;
}

pub mod login_info_function {
    use crate::model::sys::args::asys_login_info::*;
    use std::env;
    use std::{borrow::Cow, collections::HashMap};
    use user_agent_parser::UserAgentParser;

    pub fn get_user_agent_info(user_agent: &str) -> UserAgentInfo {
        let path = env::current_dir().unwrap();
        let file = path.join("config/regexes.yaml");

        let ua_parser = UserAgentParser::from_path(file).unwrap();
        let product_v = ua_parser.parse_product(user_agent);
        let os_v = ua_parser.parse_os(user_agent);
        let device_v = ua_parser.parse_device(user_agent);
        let browser = product_v.name.unwrap_or(Cow::Borrowed("")).to_string()
            + " "
            + product_v
                .major
                .unwrap_or(Cow::Borrowed(""))
                .to_string()
                .as_str();
        let os = os_v.name.unwrap_or(Cow::Borrowed("")).to_string()
            + " "
            + os_v.major.unwrap_or(Cow::Borrowed("")).to_string().as_str();
        let device = device_v.name.unwrap_or(Cow::Borrowed("")).to_string();
        UserAgentInfo {
            browser: browser.trim().to_string(),
            os: os.trim().to_string(),
            device,
        }
    }
    pub async fn get_city_by_ip(ip: &str) -> ClientNetInfo {
        let mut cliinfo = ClientNetInfo {
            ip: ip.to_string(),
            location: "Unknown City".to_owned(),
            net_work: "Unknown Network".to_owned(),
        };
        let url = "http://whois.pconline.com.cn/ipJson.jsp?json=true&ip=".to_string() + ip;
        let resp = reqwest::get(url.as_str()).await;
        let resp = match resp {
            Ok(resp) => resp,
            Err(_) => return cliinfo,
        };
        let resp = resp.text_with_charset("utf-8").await;
        if resp.is_ok() {
            let res = serde_json::from_str::<HashMap<String, String>>(resp.unwrap().as_str());
            if res.is_ok() {
                let res = res.unwrap();
                cliinfo.ip = res["ip"].to_string();
                cliinfo.location = format!("{}{}", res["pro"], res["city"]);
                cliinfo.net_work = res["addr"].split(' ').collect::<Vec<&str>>()[1].to_string();
            }
        }

        cliinfo
    }
}
