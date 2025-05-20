use reqwest::{Client, Error};

const discovery_url: &str = "https://discovery.meethue.com";

async fn post(client: &Client, url: String, body: String) -> Result<(), Error> {
    client.post(url).body(body).send().await?;
    Ok(())
}

pub async fn get_dns_data(client: Client) -> Result<String, reqwest::Error> {
    let bridge_call = client;
    let call = bridge_call.get(discovery_url);
    let response = call.send().await?.text().await;
    return response;
}
