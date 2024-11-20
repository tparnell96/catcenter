use crate::auth::Token;
use crate::config::Config;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Device {
    pub hostname: Option<String>,
    pub managementIpAddress: Option<String>,
    pub serialNumber: Option<String>,
    pub softwareVersion: Option<String>,
    // Add more fields as needed
}

#[derive(Debug, Deserialize)]
struct DevicesResponse {
    response: Vec<Device>,
}

pub async fn get_all_devices(config: &Config, token: &Token) -> Result<Vec<Device>> {
    use anyhow::anyhow;

    let client = Client::builder()
        .danger_accept_invalid_certs(!config.verify_ssl)
        .build()?;

    let devices_url = format!("{}/dna/intent/api/v1/network-device", config.dnac_url);

    let resp = client
        .get(&devices_url)
        .bearer_auth(&token.value)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "Failed to get devices: {}",
            resp.status()
        ));
    }

    let devices_resp: DevicesResponse = resp.json().await?;

    Ok(devices_resp.response)
}
