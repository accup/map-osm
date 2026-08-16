mod read_line_groups;
mod read_lines;
mod read_metadata;
mod read_points;

use std::path::Path;

use rusqlite::OpenFlags;

use crate::database_content::DatabaseContent;
use crate::geo_sqlite_error::GeoSqliteError;
use read_line_groups::read_line_groups;
use read_lines::read_lines;
use read_metadata::read_metadata;
use read_points::read_points;

/// 指定されたパスの `SQLite` データベースファイルから、データベースの内容の全体を読み出す。折れ線の種別は継承の解決を経て格納された値を返し、座標は量子化された分解能の値を返す。付帯情報とグループは格納された順序で、折れ線と点は識別子の昇順で返す。
///
/// # Errors
///
/// ファイルを開けない場合、[`crate::write_database`] が作成する構造と異なる場合、または格納された座標列を復号できない場合、エラーを返す。
pub fn read_database(path: &Path) -> Result<DatabaseContent, GeoSqliteError> {
    let connection = rusqlite::Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;

    let metadata = read_metadata(&connection)?;
    let line_groups = read_line_groups(&connection)?;
    let lines = read_lines(&connection)?;
    let points = read_points(&connection)?;

    Ok(DatabaseContent {
        metadata,
        line_groups,
        lines,
        points,
    })
}
