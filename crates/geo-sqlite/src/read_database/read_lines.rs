use polyline_codec::decode_coordinates;

use crate::coordinate_scale::COORDINATE_SCALE;
use crate::geo_sqlite_error::GeoSqliteError;
use crate::line::Line;

/// `line` テーブルから、座標列を復号した折れ線の列を識別子の昇順で読み出す。
pub(crate) fn read_lines(connection: &rusqlite::Connection) -> Result<Vec<Line>, GeoSqliteError> {
    let mut select = connection.prepare("SELECT id, kind, coordinates FROM line ORDER BY id")?;

    let rows = select
        .query_map((), |row| {
            Ok((row.get(0)?, row.get(1)?, row.get::<_, Vec<u8>>(2)?))
        })?
        .collect::<rusqlite::Result<Vec<(i64, i64, Vec<u8>)>>>()?;

    rows.into_iter()
        .map(|(id, kind, bytes)| {
            Ok(Line {
                id,
                kind: Some(kind),
                coordinates: decode_coordinates(&bytes, COORDINATE_SCALE)?,
            })
        })
        .collect()
}
