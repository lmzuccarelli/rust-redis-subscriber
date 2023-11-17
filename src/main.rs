use signal_hook::{consts::SIGINT, iterator::Signals};
use std::env;
use std::process;
use std::thread;

// define local modules
mod api;
mod handlers;
mod log;

// use local modules
use handlers::subscriber::*;
use log::logging::*;

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
        false => Level::INFO,
    };

    let log = Logging { log_level: lvl };

    if let Err(error) = process_payload() {
        log.error(&format!("{:?}", error));
        process::exit(1);
    } else {
        log.hi("connected to queue");
    }

    let mut signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("");
            log.info(&format!("received signal ctrl-c {:?}", sig));
            if sig == 2 {
                log.mid("exiting process");
                process::exit(0);
            }
        }
        loop {}
    });

    Ok(())
}
