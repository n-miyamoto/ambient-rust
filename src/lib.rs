use std::time::Duration;
use chrono::{Utc, DateTime, serde::ts_seconds_option};
use serde::{Serialize};
use reqwest::{Client, Response, Error, header::CONTENT_TYPE};
use std::fmt::{self, Formatter, Display};

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

impl Display for AmbientPayload {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl Ambient {
    pub fn new(channel_id: u32, write_key: String) -> Ambient {
        Ambient {
            channel_id,
            write_key,
            //read_key: None,
            //user_key: None,
            url: String::from("http://ambidata.io/api/v2/channels/"),
            client : reqwest::Client::new(),
        }
    }

    pub async fn send(&self, payload: &AmbientPayload, timeout_ms: Option<u64> ) -> Result<Response, Error>{
        let url: String = self.url.clone() + &self.channel_id.to_string() + "/dataarray";

        let default_timeout_ms = 10_000;
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

        // post sensor data.
        let res = self.client.post(url)
            .timeout(Duration::from_millis(timeout))
            .header(CONTENT_TYPE, "application/json")
            .body(json)
            .send()
            .await?;

        Ok(res)
    }

}
