use serde::{Deserialize, Serialize};
use serde_json::{error, from_str, to_string};

#[derive(Serialize, Deserialize, Debug)]
pub struct HueBridge {
    pub id: String,
    pub internalipaddress: String,
    pub port: u16,
}

impl HueBridge {
    pub fn from_json(pay_load: String) -> Result<Vec<HueBridge>, error::Error> {
        let deserialize: Vec<HueBridge> = from_str(&pay_load)?;
        Ok(deserialize)
    }

    pub fn to_json(&self) -> Result<String, error::Error> {
        let result = to_string(self);
        result
    }
}
