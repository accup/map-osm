mod save_to_file;

use std::path::Path;

use crate::download_error::DownloadError;
use save_to_file::save_to_file;

/// URL のリソースを HTTP GET で取得し、指定されたパスのファイルへ保存する。受信が完了した場合にのみ指定されたパスへファイルを配置し、既存のファイルは置き換える。
///
/// # Errors
///
/// リクエストの送信、エラーを示す応答ステータスの受信、応答本文の受信、ファイルの書き込みのいずれかに失敗した場合、指定されたパスを変更せずにエラーを返す。
pub fn download_to_file(url: &str, path: &Path) -> Result<(), DownloadError> {
    let mut response = ureq::get(url).call().map_err(DownloadError::new)?;

    save_to_file(&mut response.body_mut().as_reader(), path).map_err(DownloadError::new)?;

    Ok(())
}
