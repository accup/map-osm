use crate::coordinate_scale::COORDINATE_SCALE;
use crate::point::Point;

/// 点を、座標を量子化した整数値とともに `point` テーブルへ挿入する。
#[allow(
    clippy::cast_possible_truncation,
    reason = "量子化後の座標値が i64 の範囲に収まることを COORDINATE_SCALE と経緯度の値域が保証する"
)]
pub(crate) fn insert_points(
    transaction: &rusqlite::Transaction<'_>,
    points: &[Point],
) -> rusqlite::Result<()> {
    let mut insert = transaction.prepare(
        "INSERT INTO point (id, kind, reference, name, latitude, longitude)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )?;

    for point in points {
        insert.execute((
            point.id,
            point.kind,
            &point.reference,
            &point.name,
            (point.latitude * COORDINATE_SCALE).round() as i64,
            (point.longitude * COORDINATE_SCALE).round() as i64,
        ))?;
    }

    Ok(())
}
