use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::fs;
use std::sync::mpsc::channel;
use std::time::Duration;
use log::{debug, info, warn, error, trace};

fn main() -> ! {
    dotenv::dotenv().unwrap();
    env_logger::init();

    let (event_sender, event_receiver) = channel();

    let mut watcher = watcher(event_sender, Duration::from_secs(2)).unwrap();

    // TODO: configurable path
    watcher
        .watch("/home/georg/src", RecursiveMode::Recursive)
        .unwrap();

    info!("Starting...");
    loop {
        match event_receiver.recv().expect("Should not shut down") {
            DebouncedEvent::Create(ref path) => match path.as_os_str().to_str() {
                Some(path_as_str) => {
                    debug!("Received create event: {:?}", path);
                    // TODO: Other streams as well, not just the zone identifier
                    if path_as_str.ends_with(":Zone.Identifier") {
                        debug!("Removing Zone Identifier: {:?}", path);
                        match fs::remove_file(path) {
                            Ok(()) => info!("Removed Zone Identifier: {:?}", path),
                            Err(err) => error!("Could not remove zone identifier {:?} because of {:?}", path, err),
                        }
                    }
                },
                None => warn!("Could not read path: {:?}", path),
            },
            event => trace!("Ignored Event: {:?}", event),
        }
    }
}
