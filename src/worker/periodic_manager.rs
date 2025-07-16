use super::common::periodic;

pub async fn clear_periodic_worker() {
    let _ = periodic::destroy_all().await;
}
 

pub async fn periodic_worker<Args>(
    cron_str: &str,
    name: &str,
    queue: &str,
    args: Args,
    class_name: String,
) where
    Args: Sync + Send + for<'de> serde::Deserialize<'de> + serde::Serialize + 'static,
{
    let d = periodic::builder(cron_str)
        .unwrap()
        .name(name)
        .queue(queue)
        .args(args)
        .unwrap()
        .into_periodic_job(class_name)
        .unwrap();

    let payload = serde_json::to_string(&d).unwrap();
    let _ = d.update(&payload).await;
}
