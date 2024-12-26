use std::error::Error;

pub async fn get_block_tip() -> Result<u32, Box<dyn Error>> {
    let api_url = String::from("https://blockstream.info/testnet/api");
    let url = format!("{}/blocks/tip/height", api_url);
    let response = reqwest::get(url).await?.text().await?;
    Ok(response.trim().parse::<u32>()?)
}
