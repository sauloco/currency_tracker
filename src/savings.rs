extern crate reqwest;

use serde_json::Value;
use std::error::Error;

pub async fn _get_profile_id() -> Result<String, Box<dyn Error>> {
    let wise_profiles_url = dotenv!("WISE_PROFILES_URL");
    let wise_token = dotenv!("WISE_TOKEN");
    let client = reqwest::Client::new();

    let result = client
        .get(wise_profiles_url)
        .header("Authorization", wise_token)
        .send()
        .await?;

    let result: Value = result.json().await?;

    let profile_id = &result[0]["id"];

    Ok(profile_id.to_string())
}
