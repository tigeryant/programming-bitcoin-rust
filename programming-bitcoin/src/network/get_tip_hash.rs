use std::error::Error;

pub async fn get_tip_hash() -> Result<Vec<u8>, Box<dyn Error>> {
    let api_url = String::from("https://blockstream.info/testnet/api");
    let url = format!("{}/blocks/tip/hash", api_url);
    let response = reqwest::get(url).await?.text().await?;
    Ok(hex::decode(response).unwrap())
}