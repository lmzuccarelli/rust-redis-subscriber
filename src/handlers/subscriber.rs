//use redis::{ControlFlow, PubSubCommands};
use std::env;
use std::error::Error;

use crate::api::schema::*;
use crate::log::logging::*;

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
        env::set_var("REDIS_HOST", "redist://test");
        env::set_var("TOPIC", "a-simple-test");
        let tst = Mock {};
        handler(tst, log).unwrap();
    }
}
