pub mod sys_controll;
pub mod web_path;
use crate::worker::invokefunction::{InvokeFunctionMsg, InvokeFunctionWorker};
use crate::worker::AppWorker;
use axum::Router;
use web_path::WebPath;
pub struct WebApi;

impl WebApi {
    pub fn routers() -> Router {
        let mut webpath = WebPath::new();

        webpath = webpath.merge(sys_controll::router_sys());  

        webpath = webpath.final_to_path();
        let expand_path = webpath.get_last_level_paths();

        let invfun = InvokeFunctionMsg {
            job_id: None,
            callfun: "updateapi".to_owned(),
            parmets: serde_json::to_string(&expand_path).unwrap(),
        };
        tokio::spawn(async move {
            let _ = InvokeFunctionWorker::execute_async(invfun).await;
        });
        let mut router = Router::new();
        for p in expand_path {
            if let Some(method_router) = p.method_router.clone() {
                router = router.route(&p.final_path, method_router);
            }
        }

        Router::new().merge(router)
    }
    pub fn white_routers() -> Router {
        Router::new().merge(sys_controll::white_sys())
    }
}
