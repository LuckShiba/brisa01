use nickel::{Nickel, StaticFilesHandler};

pub fn start() {
    tokio::spawn(async {
        let mut server = Nickel::new();
        server.utilize(StaticFilesHandler::new("src/webserver/static"));
        server.listen("0.0.0.0:6767").expect("ok socorro");
    });
}
