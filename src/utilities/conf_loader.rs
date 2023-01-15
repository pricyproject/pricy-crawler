use crate::models::products::*;
use std::{env::current_dir, fs::read_to_string, io::Result};
pub fn config_loader(shopname: String) -> Result<MainConfig> {
    let _currenct_dir = current_dir()?.display().to_string();

    /// Loading configs from /config/shopname.toml
    /// This takes shop struct as a shopname and changes its style to snake_case by the `heck`
    /// crate).
    let complete_path = format!("configs/{shopname}.toml");

    let config_content = read_to_string(complete_path)?;

    let config: MainConfig = toml::from_str(config_content.as_str()).unwrap();

    Ok(config)
}
