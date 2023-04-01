use chrono::{Utc, DateTime};
use serde::{Serialize};
use chrono::serde::ts_seconds_option;

struct Ambient {
    channel_id : u32,
    write_key : String, 
    read_key: Option<String>, 
    user_key: Option<String>,
    url: String
}

#[derive(Serialize, Debug)]
pub struct AmbientPayload {
    #[serde(with = "ts_seconds_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,  //TODO fix time serialize format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d1: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d2: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d3: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d4: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d5: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d6: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d7: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d8: Option<f64>,
}

impl Ambient {
    fn new(channel_id: u32, write_key: String) -> Ambient {
        Ambient {
            channel_id,
            write_key,
            read_key: None,
            user_key: None,
            url: String::from("http://ambidata.io/api/v2/channels/"),
        }
    }

    fn send(&self, payload: &AmbientPayload, timeout_ms: Option<u32> ) {
        let mut url: String = self.url.clone();
        url.push_str(&self.channel_id.to_string());
        let json = serde_json::to_string(&payload).unwrap();

        let default_timeout_ms = 3000;

        let timeout = match timeout_ms {
            None => default_timeout_ms,
            Some(x) => x,
        };

        //debug pring
        println!("write key: {}", self.write_key);
        println!("json     : {}", json);
        println!("url      : {}", url);
        println!("timeout  : {} ms", timeout);

        // TODO: create request.
        // TODO: post sensor data.
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ambient = Ambient::new(123, String::from("12345"));
        let payload = AmbientPayload {
            created: Some(Utc::now()),
            d1: Some(123.4),
            d2: Some(567.8),
            d3: None,
            d4: None,
            d5: None,
            d6: None,
            d7: None,
            d8: None,
        };

        ambient.send(&payload, None);

        //assert_eq!(result, 4);
    }
}
