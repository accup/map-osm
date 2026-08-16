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

/// データベースの内容を、指定されたパスへ新規の `SQLite` データベースファイルとして書き込む。指定されたパスに既にファイルが存在する場合は削除してから書き込む。
///
/// # Errors
///
/// ファイルの削除または `SQLite` の操作に失敗した場合、および折れ線の事前条件が満たされていない場合、エラーを返す。
pub(crate) fn write_content(path: &Path, content: &DatabaseContent) -> Result<(), GeoSqliteError> {
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
