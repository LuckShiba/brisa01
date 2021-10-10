mod bot;
mod webserver;
mod websocket;

pub mod channel;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    channel::init();
    webserver::start();
    websocket::start();
    bot::start().await;
}
