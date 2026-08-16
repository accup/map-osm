use crate::coordinate_scale::COORDINATE_SCALE;
use crate::point::Point;

/// `point` テーブルから、量子化された座標を度単位へ戻した点の列を識別子の昇順で読み出す。
#[allow(
    clippy::cast_precision_loss,
    reason = "量子化された座標値は f64 の仮数部で表現できる大きさである"
)]
pub(crate) fn read_points(connection: &rusqlite::Connection) -> rusqlite::Result<Vec<Point>> {
    let mut select = connection
        .prepare("SELECT id, kind, reference, name, latitude, longitude FROM point ORDER BY id")?;

    let points = select
        .query_map((), |row| {
            Ok(Point {
                id: row.get(0)?,
                kind: row.get(1)?,
                reference: row.get(2)?,
                name: row.get(3)?,
                latitude: row.get::<_, i64>(4)? as f64 / COORDINATE_SCALE,
                longitude: row.get::<_, i64>(5)? as f64 / COORDINATE_SCALE,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(points)
}
