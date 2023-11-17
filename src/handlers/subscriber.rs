use redis::{ControlFlow, PubSubCommands};
use std::env;
use std::error::Error;

use crate::api::schema::*;
use crate::log::logging::*;

/// handler - reads json as input
pub fn process_payload() -> Result<(), Box<dyn Error>> {
    // read envars
    let h = env::var("REDIS_HOST").is_ok();
    let host = match h {
        true => env::var("REDIS_HOST").unwrap(),
        false => String::from("redis://127.0.0.1:6379"),
    };
    let tp = env::var("TOPIC").is_ok();
    let topic = match tp {
        true => env::var("TOPIC").unwrap(),
        false => String::from("test"),
    };
    let res = env::var("LOG_LEVEL").is_ok();
    // create a logging instance
    let lvl = match res {
        true => match env::var("LOG_LEVEL").unwrap().as_str() {
            "info" => Level::INFO,
            "debug" => Level::DEBUG,
            "trace" => Level::TRACE,
            _ => Level::INFO,
        },
        false => Level::INFO,
    };

    let _ = tokio::spawn(async move {
        let client = redis::Client::open(host).unwrap();
        let mut con = client.get_connection().unwrap();
        let log = &Logging { log_level: lvl };
        log.debug(&format!("subscribing to topic {:?}", topic));
        let _: () = con
            .subscribe(&[topic], |msg| {
                // the string received has to cleaned up
                // replace all \\ with blank
                // replace first and last occurance of \" with blank
                let received: String = msg.get_payload().unwrap();
                let s = received.replacen("\"", "", 1).replace("\\", "");
                let clean = s.strip_suffix("\"").unwrap();
                log.info(&format!(" clean {:#?}", &clean));
                let obj: CustomerDetails = serde_json::from_str(&clean).unwrap();
                log.info(&format!(" payload {:#?}", obj));
                return ControlFlow::Continue;
            })
            .unwrap();
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    // this brings everything from parent's scope into this scope
    use super::*;

    #[test]
    fn test_process_payload() {
        let log = &Logging {
            log_level: Level::INFO,
        };
        log.info("testing process payload");
    }
}
