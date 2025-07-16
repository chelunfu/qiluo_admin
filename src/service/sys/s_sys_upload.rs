use crate::service::prelude::*;
use base64::{engine::general_purpose, Engine};
use tokio::{fs, io::AsyncWriteExt};

pub async fn save_base64_img(base64_data: &str) -> Result<(String, String)> {
    let cleaned_str = remove_prefix(base64_data);
    let server_config = APPCOFIG.server.clone();
    let now = chrono::Local::now();
    let file_path_t = format!(
        "{}/{}",
        server_config.static_dir.clone(),
        &now.format("%Y-%m")
    );
    fs::create_dir_all(&file_path_t).await?;
    let fid = GID().await;
    let file_name = format!("{}_{}{}", now.format("%d"), fid, ".png");
    let file_path = format!("{}/{}", file_path_t, &file_name);

    let decoded_data = general_purpose::STANDARD.decode(cleaned_str)?;

    let mut file = fs::File::create(file_path).await?;
    file.write_all(&decoded_data).await?;

    let static_dir = if server_config.static_dir.starts_with("data/") {
        &server_config.static_dir["data/".len()..]
    } else {
        server_config.static_dir.as_str()
    };

    let url_path = format!(
        "{}/{}/{}/{}",
        server_config.domainname.clone(),
        static_dir,
        &now.format("%Y-%m"),
        &file_name
    );
    let no_domain_path = format!("{}/{}/{}", static_dir, &now.format("%Y-%m"), &file_name);

    Ok((url_path, no_domain_path))
}

fn remove_prefix(s: &str) -> &str {
    if let Some(send) = s.strip_prefix("data:image/png;base64,") {
        send
    } else {
        s
    }
}
pub async fn upload_file(mut multipart: Multipart) -> Result<String> {
    if let Some(field) = multipart.next_field().await? {
        let server_config = APPCOFIG.server.clone();
        let content_type = field
            .content_type()
            .map(ToString::to_string)
            .unwrap_or_else(|| "".to_string());
        let old_url = field
            .file_name()
            .map(ToString::to_string)
            .unwrap_or_else(|| "".to_string());
        let file_type = get_file_type(&content_type);
        let bytes = field.bytes().await?;
        let now = chrono::Local::now();
        let file_path_t = format!(
            "{}/{}",
            server_config.upload_dir.clone(),
            &now.format("%Y-%m")
        );
        let url_path_t = format!(
            "{}/{}",
            server_config.domainname.clone(),
            &now.format("%Y-%m")
        );
        fs::create_dir_all(&file_path_t).await?;
        let fid = GID().await;
        let file_name = format!("{}-{}{}", now.format("%d"), fid, &file_type);
        let file_path = format!("{}/{}", file_path_t, &file_name);
        let url_path = format!("{}/{}", url_path_t, &file_name);
        let mut file = fs::File::create(&file_path).await?;
        file.write_all(&bytes).await?;
        if !old_url.is_empty() {
            self::delete_file(&old_url).await;
        }
        Ok(url_path)
    } else {
        Err("Failed to upload file".into())
    }
}
pub async fn delete_file(file_path: &str) {
    let server_config = APPCOFIG.server.clone();
    let path = file_path.replace(&server_config.domainname, &server_config.upload_dir);
    match fs::remove_file(&path).await {
        Ok(_) => {}
        Err(_) => {
            tracing::error!("File deletion failed:{}", path);
        }
    }
}
fn get_file_type(content_type: &str) -> String {
    match content_type {
        "image/jpeg" => ".jpg".to_string(),
        "image/png" => ".png".to_string(),
        "image/gif" => ".gif".to_string(),
        _ => "".to_string(),
    }
}
