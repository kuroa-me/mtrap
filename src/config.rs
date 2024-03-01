use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct UrlPara {
    pub username: String,
    pub style_id: String,
    pub access_token: String,
}

impl UrlPara {
    pub fn from_file(file_path: &str) -> Result<UrlPara, Box<dyn std::error::Error + Send + Sync>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let url_para: UrlPara = toml::from_str(&contents)?;

        Ok(url_para)
    }
}
