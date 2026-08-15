mod japan_osm_pbf_path;
mod japan_osm_pbf_url;

use std::error::Error;
use std::fs;
use std::path::Path;

use crate::japan_osm_pbf_path::JAPAN_OSM_PBF_PATH;
use crate::japan_osm_pbf_url::JAPAN_OSM_PBF_URL;

fn main() -> Result<(), Box<dyn Error>> {
    let destination = Path::new(JAPAN_OSM_PBF_PATH);
    if destination.exists() {
        println!("{JAPAN_OSM_PBF_PATH} が既に存在するため、ダウンロードを省略した");
        return Ok(());
    }

    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    println!("{JAPAN_OSM_PBF_URL} をダウンロードしている");
    http_download::download_to_file(JAPAN_OSM_PBF_URL, destination)?;
    println!("{JAPAN_OSM_PBF_PATH} へ保存した");

    Ok(())
}
