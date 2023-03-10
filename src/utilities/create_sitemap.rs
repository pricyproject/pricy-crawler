use crate::models::products::ShopDetail;
use anyhow::Result;
use chrono::Utc;

use std::{
    fs::{create_dir_all, OpenOptions},
    io::prelude::*,
};
// Call now() to print current crawl time
pub fn now() -> String {
    Utc::now().date_naive().to_string()
}

// This function would create a directory inside the `./sitemaps/` dir.
// Append shopname to it with the current date and a file called sitemap.txt which contains sitemap
// links.
pub fn create_local_sitemap(
    sitemaps: &Vec<String>,
    gzip_dir: bool,
    conf: &ShopDetail,
) -> Result<()> {
    let shop_name = &conf.sitename;
    let mut path = String::new();
    let date = now();
    if gzip_dir {
        path = format!("./sitemaps/{shop_name}/gz/{date}");
    } else {
        path = format!("./sitemaps/{shop_name}/plain/{date}");
    }

    create_dir_all(&path)?;
    let file_path = format!("{path}/sitemap.txt");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)?;

    for sitemap in sitemaps {
        if let Err(e) = writeln!(file, "{sitemap}") {
            eprintln!("Couldn't write to file: {e}");
        }
    }

    println!("Sitemap created successfully on {path:?}");
    Ok(())
}
