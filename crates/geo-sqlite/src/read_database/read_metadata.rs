/// `metadata` テーブルのキーと値の組を、格納された順序で読み出す。
pub(crate) fn read_metadata(
    connection: &rusqlite::Connection,
) -> rusqlite::Result<Vec<(String, String)>> {
    let mut select = connection.prepare("SELECT key, value FROM metadata ORDER BY rowid")?;

    let metadata = select
        .query_map((), |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(metadata)
}
