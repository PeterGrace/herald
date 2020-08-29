use reqwest::{
    Client,
    Response
};
use std::error;

pub async fn send_hook(url: String, method: String, body: String) -> Result<Response, Box<dyn error::Error>>  {
    let client = Client::new();
    match method.to_lowercase().as_str() {
        "get" => {
            let result = client.get(&url).send().await;
            let response = result.unwrap();
            info!("response code: {}",response.status());
            Ok(response)
            }
        "post" => {
            info!("Body to be sent is: {}", body);
            let result = client.post(&url).body(body).send().await;
            let response = result.unwrap();
            info!("response code: {}",response.status());
            Ok(response)
        }
        _ => {
            Err("unknown request type".into())
        }
    }
}