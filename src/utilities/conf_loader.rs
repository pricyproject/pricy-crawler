use crate::models::products::*;
use std::{fs::read_to_string, io::Result};
/// Loading configs from `/config/shopname.toml`
/// This takes shop struct as a shopname and changes its style to snake_case by the `heck`
/// crate).
pub fn config_loader(shopname: String) -> Result<MainConfig> {
    let complete_path = format!("configs/{shopname}.toml");

    let config_content = read_to_string(complete_path)?;

    let config: MainConfig = toml::from_str(config_content.as_str()).unwrap();

    Ok(config)
}
