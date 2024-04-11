mod reader;
mod watcher;

use std::sync::{Arc, Mutex};

use reader::ScaleReader;
use watcher::ScaleWatcher;

#[tokio::main(flavor = "multi_thread")]

async fn main() {
    let port_name = Arc::new(Mutex::new(String::new()));
    let reader_port = port_name.clone();

    tokio::task::spawn(ScaleWatcher::new(port_name));
    tokio::task::spawn(ScaleReader::new(reader_port));

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
