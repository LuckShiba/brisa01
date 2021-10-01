use std::{io::Read};

use nickel::{HttpRouter, Nickel, StaticFilesHandler};
use multipart::server::nickel::MultipartBody;

// use crate::channel::ChannelElement;

pub fn start() {
    tokio::spawn(async {
        let mut server = Nickel::new();
        server.utilize(StaticFilesHandler::new("src/webserver/static"));
        server.post("/send", middleware!{ |req|
            // let mut message: ChannelElement;
            if let Some(mut multipart) = (*req).multipart_body() {
                if let Ok(Some(mut entry)) = multipart.read_entry() {
                    if !entry.is_text() {
                        let filename = entry.headers.filename.unwrap_or_else(|| "unknown.txt".into());
                        let mut buffer = Vec::new();
                        entry.data.read_to_end(&mut buffer).ok();
                        println!("{}: {:?}", filename, buffer.len())
                    }
                }
            }

            "foda"
        });
        server.listen("0.0.0.0:6767").expect("ok socorro");
    });
}
