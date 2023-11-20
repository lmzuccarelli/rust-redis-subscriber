use redis::{ControlFlow, PubSubCommands};
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::env;
use std::error::Error;
use std::process;
use std::thread;

// define local modules
mod api;
mod handlers;
mod log;

// use local modules
use api::schema::*;
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
        false => Level::TRACE,
    };

    let log = Logging { log_level: lvl };
    impl MessageQueueInterface for ImplMessageQueueInterface {
        fn subscribe(
            &self,
            log: &Logging,
            host: String,
            topic: String,
        ) -> Result<(), Box<dyn Error>> {
            let client = redis::Client::open(host).unwrap();
            let mut con = client.get_connection().unwrap();
            log.debug(&format!("subscribing to topic {:?}", topic));
            let _: () = con
                .subscribe(&[topic], |msg| {
                    // the string received has to cleaned up
                    // replace all \\ with blank
                    // replace first and last occurance of \" with blank
                    let received: String = msg.get_payload().unwrap();
                    let s = received.replacen("\"", "", 1).replace("\\", "");
                    let clean = s.strip_suffix("\"").unwrap();
                    log.debug(&format!("clean {:#?}", &clean));
                    let obj: CustomerDetails = serde_json::from_str(&clean).unwrap();
                    log.info(&format!("payload {:#?}", obj));
                    return ControlFlow::Continue;
                })
                .unwrap();
            Ok(())
        }
    }

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
