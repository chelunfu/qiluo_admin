use qiluo::app::App;
use qiluo::banner::BANNER;
#[tokio::main]
async fn main() {
    println!("{}", BANNER);
    App::run().await;
}
