use env_logger;
use log::{info,trace};
use std::env;
mod audio;

fn main() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    trace!("main()");
    info!("ALSA TEST");

    match audio::init() {
        Ok(_) => info!("init() success"),
        Err(e) => panic!("audio::init() failed: {}", e),
    }
}
