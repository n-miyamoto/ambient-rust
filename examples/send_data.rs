use ambient_rust::{Ambient, AmbientPayload};
use reqwest::{StatusCode, Error};

mod secrets;

#[tokio::main]
async fn main() -> Result<(), Error> {
    //const CHANNEL_ID: u32 = 12345;
    //const WRITE_KEY: &str = "1234";
    let dummy_data = vec![12.3, 45.6, 78.9];

    let ambient = Ambient::new(secrets::ambient::CHANNEL_ID, String::from(secrets::ambient::WRITE_KEY));
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

    let response = ambient.send(&payload, None).await?;

    match response.status() {
        StatusCode::OK => println!("success!"),
        StatusCode::PAYLOAD_TOO_LARGE => {
            println!("Request payload is too large!");
        }
        s => println!("Received response status: {:?}", s),
    };
    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}
