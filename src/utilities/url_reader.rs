use crate::shops::ShopName;
use anyhow::Result;
use heck::ToSnakeCase;
use regex::Regex;
use reqwest::Url;
use std::{
    fs::read_to_string,
    io::{stdin, Read},
};
use walkdir::WalkDir;

pub async fn url_loader_from_pipe() -> Result<()> {
    let url_regex = Regex::new(
        r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)",
    )?;
    let config_directory_files = WalkDir::new("./configs/");
    let mut buffer = String::new();
    let mut standard_input = stdin();
    standard_input.read_to_string(&mut buffer)?;

    // Gather valid urls.
    let mut valid_urls = Vec::new();
    for input_line in buffer.lines() {
        if let Some(valid_url) = url_regex.captures(input_line) {
            valid_urls.push(valid_url.get(0).unwrap().as_str())
        }
    }
    // Parse config files in the `config` directory.
    // Extract `site_address` values and push them to `site_addresses` array.
    let mut site_addresses = Vec::new();
    for file in config_directory_files.into_iter().skip(1) {
        let path = file?.path().display().to_string();
        let content = read_to_string(path)?;
        for line in content.lines() {
            if line.contains("site_address") {
                let site_address = line.replace("site_address =", "").replace(['"', ' '], "");
                let parsed_url = Url::parse(site_address.as_str())?;
                let base_host = parsed_url.host_str().unwrap().to_owned();
                let bb = base_host.clone();
                site_addresses.push(bb)
            }
        }
    }

    // Check URLs inside `valid_urls` with site_address.
    // And push them to crawlable URls'
    let mut crawlable_urls: Vec<&str> = Vec::new();
    for url in valid_urls.iter() {
        let parsed_url = Url::parse(url)?;
        let base_host = parsed_url.host_str().unwrap();
        if site_addresses.contains(&base_host.to_string()) && !crawlable_urls.contains(url) {
            crawlable_urls.push(url)
        }
    }

    // Crawl each url based on their hosts
    for valid_url in valid_urls {
        let shop_name = Url::parse(valid_url)?.host_str().unwrap().to_owned();
        let new_shop_name: ShopName = shop_name.replace("www.", "").to_snake_case().parse()?;
        new_shop_name.crawl_single_url(valid_url).await?
    }

    Ok(())
}
