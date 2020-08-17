pub async fn send_hook(url: String, method: String, body: String) -> ()  {
    let client = reqwest::Client::new();
    match method.to_lowercase().as_str() {
        "get" => {
            let res = client.get(&url).send().await;
            info!("response code: {}",res.unwrap().status())
            }
        "post" => {
            info!("Body to be sent is: {}", body);
            let res = client.post(&url).body(body).send().await;
            info!("response code: {}",res.unwrap().status())
        }
        _ => info!("Unknown request type: {}", method)
    }
}