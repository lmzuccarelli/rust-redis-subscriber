use crate::handlers::subscriber::{handler, ImplMessageQueueInterface};
use custom_logger::*;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::env;
use std::process;
use std::thread;

// define local modules
mod api;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = env::var("LOG_LEVEL").is_ok();
    let lvl = match &res {
        true => match env::var("LOG_LEVEL").unwrap().as_str() {
            "info" => Level::INFO,
            "debug" => Level::DEBUG,
            "trace" => Level::TRACE,
            _ => Level::INFO,
        },
        false => Level::TRACE,
    };

    let log = Logging { log_level: lvl };
    let impl_q = ImplMessageQueueInterface {};
    if let Err(error) = handler(impl_q, &log) {
        log.error(&format!("{:?}", error));
        process::exit(1);
    } else {
        log.info("connected to queue");
    }

    let mut signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("");
            log.info(&format!("received signal ctrl-c {:?}", sig));
            if sig == 2 {
                log.info("exiting queue subscribe process");
                process::exit(0);
            }
        }
        loop {}
    });

    Ok(())
}
