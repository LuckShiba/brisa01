mod webserver;
mod websocket;
mod bot;

pub mod channel;

use dotenv::dotenv;

#[macro_use]
extern crate nickel;

#[tokio::main]
async fn main() {
    dotenv().ok();
    channel::init();
    webserver::start();
    websocket::start();
    bot::start().await;
}
