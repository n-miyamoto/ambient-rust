use chrono::{Utc, DateTime};
use chrono::serde::ts_seconds_option;

use serde::{Serialize};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;
pub struct Ambient {
    channel_id : u32,
    write_key : String, 
    //read_key: Option<String>, 
    //user_key: Option<String>,
    url: String, 
    client: Client,
}

#[derive(Serialize, Debug, Clone)]
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
    pub fn new(channel_id: u32, write_key: String) -> Ambient {
        Ambient {
            channel_id,
            write_key,
            //read_key: None,
            //user_key: None,
            url: String::from("http://ambidata.io/api/v2/channels/"),
            client : reqwest::blocking::Client::new(),
        }
    }

    pub fn send(&self, payload: &AmbientPayload, timeout_ms: Option<u32> ) {
        let url: String = self.url.clone() + &self.channel_id.to_string() + "/dataarray";

        let default_timeout_ms = 3000;
        let timeout = match timeout_ms {
            None => default_timeout_ms,
            Some(x) => x,
        };

        #[allow(non_snake_case)]
        #[derive(Serialize, Debug)]
        struct DataArray{
            writeKey: String,
            data: Vec<AmbientPayload> ,
        }
        let data_array = DataArray{
            writeKey : self.write_key.clone(),
            data : vec![payload.clone(); 1],
        };

        let json = serde_json::to_string(&data_array).unwrap();

        //debug print
        println!("write key: {}", self.write_key);
        println!("json     : {}", json);
        println!("url      : {}", url);
        println!("timeout  : {} ms", timeout);

        // post sensor data.
        let _res = self.client.post(url)
            .header(CONTENT_TYPE, "application/json")
            .body(json)
            .send();
        //match res.status() {
        //    StatusCode::OK => println!("success!"),
        //    StatusCode::PAYLOAD_TOO_LARGE => {
        //        println!("Request payload is too large!");
        //    }
        //    s => println!("Received response status: {:?}", s),
        //};
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        const CHANNEL_ID: u32 = 12345;
        const WRITE_KEY: &str = "12345";

        let ambient = Ambient::new(CHANNEL_ID, String::from(WRITE_KEY));
        let payload = AmbientPayload {
            //created: Some(Utc::now()),
            created: None,
            d1: Some(12.3),
            d2: Some(45.6),
            d3: Some(78.9),
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
