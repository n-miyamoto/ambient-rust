use std::time::Duration;
use chrono::{Utc, DateTime, serde::ts_seconds_option};
use serde::{Serialize};
use reqwest::{blocking::{Client, Response}, Error, header::CONTENT_TYPE};

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

    pub fn send(&self, payload: &AmbientPayload, timeout_ms: Option<u64> ) -> Result<Response, Error>{
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
            .send()?;

        Ok(res)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;

    #[test]
    fn it_works() {
        const CHANNEL_ID: u32 = 12345;
        const WRITE_KEY: &str = "1234";
        let dummy_data = vec![12.3, 45.6, 78.9];

        let ambient = Ambient::new(CHANNEL_ID, String::from(WRITE_KEY));
        let payload = AmbientPayload {
            //created: Some(Utc::now()), Persing chrono::DataTime is not supported yes.
            created: None,
            d1: Some(dummy_data[0]),
            d2: Some(dummy_data[1]),
            d3: Some(dummy_data[2]),
            d4: None,
            d5: None,
            d6: None,
            d7: None,
            d8: None,
        };

        let response = ambient.send(&payload, None);
        match &response{
            Ok(res) =>  {
                match res.status() {
                    StatusCode::OK => println!("success!"),
                    StatusCode::PAYLOAD_TOO_LARGE => {
                        println!("Request payload is too large!");
                    }
                    s => println!("Received response status: {:?}", s),
                };
            },
            Err(error) => {
                panic!("Http post failled.: {:?}", error);
            }
        }

        assert_eq!(response.unwrap().status(), StatusCode::NOT_FOUND);
    }
}
