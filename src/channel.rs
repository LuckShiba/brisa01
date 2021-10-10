use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc,
};

use serde::{Deserialize, Serialize};
use serenity::model::channel::Message as DiscordMessage;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub avatar_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub author: User,
}

impl From<DiscordMessage> for Message {
    fn from(val: DiscordMessage) -> Self {
        Message {
            author: User {
                username: val.author.tag(),
                avatar_url: val
                    .author
                    .avatar_url()
                    .unwrap_or_else(|| val.author.default_avatar_url()),
            },
            content: val.content,
        }
    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
        format!("{}: {}", self.author.username, self.content)
    }
}

pub type ChannelElement = Message;
pub type Channel = Arc<(Sender<ChannelElement>, Receiver<ChannelElement>)>;

static mut BOT_CHANNEL: Option<Channel> = None;
static mut WS_CHANNEL: Option<Channel> = None;

pub fn init() {
    unsafe {
        BOT_CHANNEL = Some(Arc::new(channel()));
        WS_CHANNEL = Some(Arc::new(channel()));
    }
}

pub fn get_bot_channel() -> Channel {
    unsafe { BOT_CHANNEL.clone().unwrap() }
}

pub fn get_ws_channel() -> Channel {
    unsafe { WS_CHANNEL.clone().unwrap() }
}

pub fn send(ch: &Channel, el: ChannelElement) {
    ch.0.send(el).unwrap();
}

pub fn receive(ch: &Channel) -> ChannelElement {
    ch.1.recv().unwrap()
}
