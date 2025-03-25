use crate::api::schema::CustomerDetails;
use custom_logger::*;
use redis::{ControlFlow, PubSubCommands};
use std::env;
use std::error::Error;

pub struct ImplMessageQueueInterface {}

pub trait MessageQueueInterface {
    // used to interact with container registry (manifest calls)
    fn subscribe(&self, log: &Logging, host: String, topic: String) -> Result<(), Box<dyn Error>>;
}

impl MessageQueueInterface for ImplMessageQueueInterface {
    fn subscribe(&self, log: &Logging, host: String, topic: String) -> Result<(), Box<dyn Error>> {
        log.trace(&format!("host {}", &host));
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

/// handler - reads json as input
pub fn handler<T: MessageQueueInterface>(q: T, log: &Logging) -> Result<(), Box<dyn Error>> {
    // read envars
    let h = env::var("REDIS_HOST").is_ok();
    let host = match h {
        true => env::var("REDIS_HOST").unwrap(),
        false => {
            log.warn("envar REDIS_HOST is not set using default");
            String::from("redis://127.0.0.1:6379")
        }
    };
    let tp = env::var("TOPIC").is_ok();
    let topic = match tp {
        true => env::var("TOPIC").unwrap(),
        false => {
            log.trace("envar TOPIC is not set using default");
            String::from("test")
        }
    };
    q.subscribe(log, host, topic).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    // this brings everything from parent's scope into this scope
    use super::*;

    struct Mock {}

    impl MessageQueueInterface for Mock {
        fn subscribe(
            &self,
            log: &Logging,
            _host: String,
            _topic: String,
        ) -> Result<(), Box<dyn Error>> {
            log.info("testing queue subscribe");
            log.info("{ \"test\":\"test\" }");
            Ok(())
        }
    }

    #[test]
    fn test_handler_no_vars_pass() {
        let log = &Logging {
            log_level: Level::INFO,
        };
        let tst = Mock {};
        handler(tst, log).unwrap();
    }

    #[test]
    fn test_handler_set_vars_pass() {
        let log = &Logging {
            log_level: Level::INFO,
        };
        //env::set_var("REDIS_HOST", "redist://test");
        //env::set_var("TOPIC", "a-simple-test");
        let tst = Mock {};
        handler(tst, log).unwrap();
    }
}
