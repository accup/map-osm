mod create_schema;
mod insert_line_groups;
mod insert_lines;
mod insert_metadata;
mod insert_points;
mod resolve_line_kinds;

use std::path::Path;

use crate::database_content::DatabaseContent;
use crate::geo_sqlite_error::GeoSqliteError;
use create_schema::create_schema;
use insert_line_groups::insert_line_groups;
use insert_lines::insert_lines;
use insert_metadata::insert_metadata;
use insert_points::insert_points;
use resolve_line_kinds::resolve_line_kinds;

/// データベースの内容を、指定されたパスへ `SQLite` データベースとして書き込む。事前条件として、指定されたパスにはファイルが存在しないか、空のファイルが存在すること。
///
/// # Errors
///
/// `SQLite` の操作に失敗した場合、および折れ線の事前条件が満たされていない場合、エラーを返す。
pub(crate) fn write_content(path: &Path, content: &DatabaseContent) -> Result<(), GeoSqliteError> {
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
