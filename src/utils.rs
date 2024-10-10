use std::error::Error;
use std::time::Duration;

pub fn is_png(data: &[u8]) -> bool {
    data.len() >= 8 && &data[0..8] == [137, 80, 78, 71, 13, 10, 26, 10]
}

pub async fn download_png(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;
    let bytes = bytes.to_vec();
    if !is_png(&bytes) {
        return Err("downloaded image is not a png".into());
    }
    Ok(bytes)
}

