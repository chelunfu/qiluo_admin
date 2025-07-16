use crate::service::prelude::*;

pub async fn analysis_total() -> impl IntoResponse {
    let body = json!({
        "users":404,
        "messages": 400,
        "moneys": 500,
        "shoppings": 666
    });
    ApiResponse::ok(body)
}
pub async fn user_access_source() -> impl IntoResponse {
    let body = json!([
      { "value": 1000, "name": "analysis.directAccess" },
      { "value": 310, "name": "analysis.mailMarketing" },
      { "value": 234, "name": "analysis.allianceAdvertising" },
      { "value": 135, "name": "analysis.videoAdvertising" },
      { "value": 1548, "name": "analysis.searchEngines" }
    ]);
    ApiResponse::ok(body)
}
pub async fn weekly_user_activity() -> impl IntoResponse {
    let body = json!( [
        { "value": 13253, "name": "analysis.monday" },
        { "value": 34235, "name": "analysis.tuesday" },
        { "value": 26321, "name": "analysis.wednesday" },
        { "value": 12340, "name": "analysis.thursday" },
        { "value": 24643, "name": "analysis.friday" },
        { "value": 1322, "name": "analysis.saturday" },
        { "value": 1324, "name": "analysis.sunday" }
      ]
    );
    ApiResponse::ok(body)
}
pub async fn monthly_sales() -> impl IntoResponse {
    let body = json!( [
      { "estimate": 100, "actual": 120, "name": "analysis.january" },
      { "estimate": 120, "actual": 82, "name": "analysis.february" },
      { "estimate": 161, "actual": 91, "name": "analysis.march" },
      { "estimate": 134, "actual": 154, "name": "analysis.april" },
      { "estimate": 105, "actual": 162, "name": "analysis.may" },
      { "estimate": 160, "actual": 140, "name": "analysis.june" },
      { "estimate": 165, "actual": 145, "name": "analysis.july" },
      { "estimate": 114, "actual": 250, "name": "analysis.august" },
      { "estimate": 163, "actual": 134, "name": "analysis.september" },
      { "estimate": 185, "actual": 56, "name": "analysis.october" },
      { "estimate": 118, "actual": 99, "name": "analysis.november" },
      { "estimate": 123, "actual": 123, "name": "analysis.december" }
    ]);
    ApiResponse::ok(body)
}

pub async fn workplace_total() -> impl IntoResponse {
    let body = json!({
        "project":985,
        "access": 8795,
        "todo":1879
    });
    ApiResponse::ok(body)
}

pub async fn workplace_project() -> impl IntoResponse {
    let now = Local::now().naive_local();
    let body = json!([{
        "name": "Github",
        "icon": "akar-icons:github-fill",
        "message": "workplace.introduction",
        "personal": "fff",
        "time":now
    }]);
    ApiResponse::ok(body)
}

pub async fn workplace_dynamic() -> impl IntoResponse {
    let now = Local::now().naive_local();
    let body = json!([{
        "keys": json!(["workplace.push","message" ]),
        "time": now
    },{
        "keys": json!(["workplace.push","message" ]),
        "time": now
    }]);
    ApiResponse::ok(body)
}

pub async fn workplace_team() -> impl IntoResponse {
    let body = json!([{
        "name":  "Github",
        "icon": "icon"
    },{
        "name":  "Vue",
        "icon": "icon"
    }]);
    ApiResponse::ok(body)
}

pub async fn workplace_radar() -> impl IntoResponse {
    let body = json!([{
        "name":  "workplace.quote",
        "max":65,
        "personal":42,
        "team":50
    },{
        "name":  "workplace.contribution",
        "max":65,
        "personal":42,
        "team":50
    }]);
    ApiResponse::ok(body)
}
