use std::thread;

use ws::{Message as WSMessage, listen};
use crate::channel::{ChannelElement, get_bot_channel, get_ws_channel, receive, send};

pub fn start() {
    tokio::spawn(async {
        let mut should_receive = true;
        listen("0.0.0.0:3012", |out| {
            if should_receive {
                should_receive = false;
                let clone = out.clone();
                thread::spawn(move || {
                    loop {
                        let msg = receive(&get_ws_channel());
                        if let Err(why) = clone.broadcast(serde_json::to_string(&msg).unwrap()) {
                            eprintln!("{:?}", why);
                        }
                    }
                });
            }

            move |msg: WSMessage| {
                match serde_json::from_str::<ChannelElement>(msg.to_string().as_str()) {
                    Ok(msg) => {
                        send(&get_bot_channel(), msg);
                    }
                    Err(_) => {
                        out.close(ws::CloseCode::Unsupported).ok();
                    }
                }
                Ok(())
            }
        }).expect("assim amigo... nao foi");
    });
}
