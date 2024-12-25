use std::error::Error;

pub fn get_block_tip() -> Result<u32, Box<dyn Error>> {
    let api_url = String::from("https://blockstream.info/testnet/api");
    let url = format!("{}/blocks/tip/height", api_url);
    let response = reqwest::blocking::get(url)?.text()?;
    let raw = hex::decode(response.trim())?.try_into().unwrap(); // to little endian u32??
    // from big or little endian??
    dbg!(u32::from_be_bytes(raw));
    Ok(u32::from_be_bytes(raw))
}