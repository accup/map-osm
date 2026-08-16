/// 付帯情報のキーと値の組を `metadata` テーブルへ挿入する。
pub(crate) fn insert_metadata(
    transaction: &rusqlite::Transaction<'_>,
    metadata: &[(String, String)],
) -> rusqlite::Result<()> {
    let mut insert = transaction.prepare("INSERT INTO metadata (key, value) VALUES (?1, ?2)")?;

    for (key, value) in metadata {
        insert.execute((key, value))?;
    }

    Ok(())
}
