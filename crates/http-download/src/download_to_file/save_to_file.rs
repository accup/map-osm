use std::fs;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

/// リーダーから読み出したすべてのバイト列を、指定されたパスのファイルへ保存する。書き込みの完了までは同じディレクトリの一時ファイルを使用し、完了した場合にのみ指定されたパスへ改名して配置する。失敗した場合は一時ファイルを削除し、指定されたパスを変更しない。
pub(crate) fn save_to_file(reader: &mut impl Read, path: &Path) -> io::Result<()> {
    let mut temporary_name = path.as_os_str().to_owned();
    temporary_name.push(".download");
    let temporary_path = PathBuf::from(temporary_name);

    let mut file = fs::File::create(&temporary_path)?;
    let copy_result = io::copy(reader, &mut file);

    if copy_result.is_err() {
        let _ = fs::remove_file(&temporary_path);
    }
    copy_result?;

    fs::rename(&temporary_path, path)?;

    Ok(())
}
