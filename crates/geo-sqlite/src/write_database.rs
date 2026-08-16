mod write_content;

use std::path::Path;

use crate::database_content::DatabaseContent;
use crate::geo_sqlite_error::GeoSqliteError;
use write_content::write_content;

/// データベースの内容を、指定されたパスの `SQLite` データベースファイルへ書き込む。同じディレクトリに排他的に作成した一意な名前の一時ファイルへ書き込みを完了した場合にのみ、指定されたパスへ改名して既存のファイルを置き換える。事後条件として、種別を持たない折れ線には、その折れ線を構成要素に含む最初のグループの種別を格納する。グループの構成要素のうち折れ線の列に存在しない識別子は無視し、重複する識別子は 1 つに纏める。事前条件として、各折れ線は 1 点以上の座標を持ち、種別を指定されているか種別を持つグループに属していること。
///
/// # Errors
///
/// ファイルの操作または `SQLite` の操作に失敗した場合、および折れ線の事前条件が満たされていない場合、一時ファイルを削除し、指定されたパスを変更せずにエラーを返す。
pub fn write_database(path: &Path, content: &DatabaseContent) -> Result<(), GeoSqliteError> {
    let directory = match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent,
        _ => Path::new("."),
    };
    let temporary = tempfile::NamedTempFile::new_in(directory)?;

    write_content(temporary.path(), content)?;

    temporary
        .persist(path)
        .map_err(|failure| GeoSqliteError::from(failure.error))?;

    Ok(())
}
