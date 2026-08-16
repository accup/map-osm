mod create_schema;
mod insert_line_groups;
mod insert_lines;
mod insert_metadata;
mod insert_points;
mod resolve_line_kinds;

use std::fs;
use std::io;
use std::path::Path;

use crate::database_content::DatabaseContent;
use crate::geo_sqlite_error::GeoSqliteError;
use create_schema::create_schema;
use insert_line_groups::insert_line_groups;
use insert_lines::insert_lines;
use insert_metadata::insert_metadata;
use insert_points::insert_points;
use resolve_line_kinds::resolve_line_kinds;

/// データベースの内容を、指定されたパスの `SQLite` データベースファイルへ書き込む。既存のファイルは置き換える。事後条件として、種別を持たない折れ線には、その折れ線を構成要素に含む最初のグループの種別を格納する。グループの構成要素のうち折れ線の列に存在しない識別子は無視し、重複する識別子は 1 つに纏める。事前条件として、各折れ線は 1 点以上の座標を持ち、種別を指定されているか種別を持つグループに属していること。
///
/// # Errors
///
/// ファイルの置き換えまたは `SQLite` の操作に失敗した場合、および折れ線の事前条件が満たされていない場合、エラーを返す。
pub fn write_database(path: &Path, content: &DatabaseContent) -> Result<(), GeoSqliteError> {
    match fs::remove_file(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }

    let mut connection = rusqlite::Connection::open(path)?;
    create_schema(&connection)?;

    let line_kinds = resolve_line_kinds(&content.lines, &content.line_groups)?;

    let transaction = connection.transaction()?;
    insert_metadata(&transaction, &content.metadata)?;
    insert_lines(&transaction, &content.lines, &line_kinds)?;
    insert_line_groups(&transaction, &content.line_groups, &content.lines)?;
    insert_points(&transaction, &content.points)?;
    transaction.commit()?;

    Ok(())
}
