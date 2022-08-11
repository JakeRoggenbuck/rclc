use super::structures::DiscoveryRequest;
use super::reqwest;

pub struct DiscoveryServerConfig {
    pub url: String,
}

async fn discover(disc_conf: DiscoveryServerConfig, disc_request: DiscoveryRequest) {
    let client = reqwest::Client::new();
    let res = client
        .post(disc_conf.url)
        .body("Hello!")
        .send()
        .await
        .expect("Could not await");

    println!("res = {:?}", res);
}
