use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::japan_osm_pbf_path::JAPAN_OSM_PBF_PATH;
use crate::japan_osm_pbf_url::JAPAN_OSM_PBF_URL;

/// 日本全域の OSM PBF ファイルの存在を保証し、そのパスを返す。ファイルが存在しない場合はダウンロードして保存する。
///
/// # Errors
///
/// 保存先のディレクトリの作成またはダウンロードに失敗した場合、エラーを返す。
pub fn ensure_japan_osm_pbf() -> Result<PathBuf, Box<dyn Error>> {
    let destination = PathBuf::from(JAPAN_OSM_PBF_PATH);
    if destination.exists() {
        println!("{JAPAN_OSM_PBF_PATH} が既に存在するため、ダウンロードを省略した");
        return Ok(destination);
    }

    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    println!("{JAPAN_OSM_PBF_URL} をダウンロードしている");
    http_download::download_to_file(JAPAN_OSM_PBF_URL, &destination)?;
    println!("{JAPAN_OSM_PBF_PATH} へ保存した");

    Ok(destination)
}
