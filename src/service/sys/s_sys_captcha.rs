use crate::model::sys::args::acaptch::*;
use crate::service::prelude::*;
use captcha_rust::Captcha;
use image::{DynamicImage, ImageBuffer};
pub async fn get_captcha(VQuery(arg): VQuery<ClientInfo>) -> impl IntoResponse {
    let res: CaptchaImage = gen_captcha(arg).await;
    ApiResponse::ok(res)
}

async fn gen_captcha(arg: ClientInfo) -> CaptchaImage {
    let captcha = Captcha::new(5, 130, 40);

    let cacheinfo=CaptchaCacheInfo {
        client_id: arg.client_id,
        cache_text: captcha.text.clone(),
    };
    let uuid = GID().await;
    let cache = CacheManager::instance().await;
    let _ = cache
        .set_value_ex(&format!("capcha:{}", uuid), &cacheinfo, 300)
        .await;
    info!("获取验证码:{}", captcha.text);
    CaptchaImage {
        captcha_on_off: true,
        uuid,
        img: captcha.base_img,
    }
}

pub async fn gen_wx_captcha() -> String {
    let captcha = Captcha::new(5, 130, 40);
    let captchtxt = captcha.text;
    let wxcaptcha = format!("wx_{}", captchtxt);
    let cache = CacheManager::instance().await;
    let _ = cache
        .set_string_ex(&wxcaptcha, captchtxt.as_str(), 300)
        .await;
    captchtxt
}

//滑动验证码生成
async fn solid_captcha() {
    let width = 520;
    let height = 320;
    let mut img = image::open("original_captcha.png").unwrap().into_rgba8();

    let mut mask_img = ImageBuffer::new(50, 50);
    let mask_image = image::open("mask.png").unwrap().into_luma8();
    // 确定缺口的位置和大小
    let gap_x = 300; // rng.gen_range(50..450);
    let gap_y = 200; //rng.gen_range(50..250);
    let gap_width = 50;
    let gap_height = 50;
    // 绘制缺口（将缺口区域的像素设置为白色）
    for x in gap_x..(gap_x + gap_width) {
        for y in gap_y..(gap_y + gap_height) {
            if mask_image.get_pixel(x - gap_x, y - gap_y)[0] == 255 {
                let mut pixel = *img.get_pixel(x, y);
                mask_img.put_pixel(x - gap_x, y - gap_y, pixel);
                pixel[3] = 150;
                img.put_pixel(x, y, pixel);
            }
        }
    }
    let mask = DynamicImage::ImageRgba8(mask_img).as_bytes();

    DynamicImage::ImageRgba8(img).as_bytes();
}
