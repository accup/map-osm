/// 接続先のデータベースへ、付帯情報・折れ線・グループ・グループ構成・空間索引・点の各テーブルを作成する。
pub(crate) fn create_schema(connection: &rusqlite::Connection) -> rusqlite::Result<()> {
    connection.execute_batch(
        "CREATE TABLE metadata (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        CREATE TABLE line_group (
            id INTEGER PRIMARY KEY,
            kind INTEGER NOT NULL,
            reference TEXT,
            name TEXT
        );
        CREATE TABLE line (
            id INTEGER PRIMARY KEY,
            kind INTEGER NOT NULL,
            coordinates BLOB NOT NULL
        );
        CREATE TABLE line_group_member (
            group_id INTEGER NOT NULL REFERENCES line_group (id),
            line_id INTEGER NOT NULL REFERENCES line (id),
            PRIMARY KEY (group_id, line_id)
        );
        CREATE VIRTUAL TABLE line_index USING rtree(
            id, min_latitude, max_latitude, min_longitude, max_longitude
        );
        CREATE TABLE point (
            id INTEGER PRIMARY KEY,
            kind INTEGER NOT NULL,
            reference TEXT,
            name TEXT,
            latitude INTEGER NOT NULL,
            longitude INTEGER NOT NULL
        );",
    )
}
