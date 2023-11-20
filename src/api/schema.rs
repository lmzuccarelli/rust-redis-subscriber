// module schema

use crate::log::logging::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomerDetails {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "surname")]
    pub surname: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "mobile")]
    pub mobile: String,
}

pub struct ImplMessageQueueInterface {}

pub trait MessageQueueInterface {
    // used to interact with container registry (manifest calls)
    fn subscribe(&self, log: &Logging, host: String, topic: String) -> Result<(), Box<dyn Error>>;
}
