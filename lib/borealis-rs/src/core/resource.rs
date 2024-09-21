use anyhow::anyhow;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resources/"]
struct Resource;

pub fn read_to_string(file_path: &str) -> anyhow::Result<String> {
    if let Some(file) = Resource::get(file_path) {
        return Ok(String::from_utf8_lossy(&file.data).to_string());
    }
    Err(anyhow!("File {} not found", file_path))
}

pub fn read_to_bytes(file_path: &str) -> anyhow::Result<Vec<u8>> {
    if let Some(file) = Resource::get(file_path) {
        return Ok(file.data.to_vec());
    }
    Err(anyhow!("File {} not found", file_path))
}