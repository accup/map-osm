mod coordinate_bounds;
mod quantize_coordinates;

use polyline_codec::encode_coordinates;

use crate::coordinate_scale::COORDINATE_SCALE;
use crate::geo_sqlite_error::GeoSqliteError;
use crate::line::Line;
use coordinate_bounds::coordinate_bounds;
use quantize_coordinates::quantize_coordinates;

/// 折れ線を、符号化した座標列とともに `line` テーブルへ挿入し、その外接矩形を空間索引 `line_index` へ挿入する。外接矩形は、格納される座標を索引が包含するよう量子化後の座標から算出する。種別は折れ線の列と同じ順序の解決済みの種別の列から与える。
///
/// # Errors
///
/// 座標を持たない折れ線が存在する場合、または `SQLite` の操作に失敗した場合、エラーを返す。
pub(crate) fn insert_lines(
    transaction: &rusqlite::Transaction<'_>,
    lines: &[Line],
    line_kinds: &[i64],
) -> Result<(), GeoSqliteError> {
    let mut insert_line =
        transaction.prepare("INSERT INTO line (id, kind, coordinates) VALUES (?1, ?2, ?3)")?;
    let mut insert_index = transaction.prepare(
        "INSERT INTO line_index (id, min_latitude, max_latitude, min_longitude, max_longitude)
        VALUES (?1, ?2, ?3, ?4, ?5)",
    )?;

    for (line, &kind) in lines.iter().zip(line_kinds) {
        let quantized = quantize_coordinates(&line.coordinates);
        let bounds = coordinate_bounds(&quantized).ok_or(GeoSqliteError::EmptyLine(line.id))?;

        insert_line.execute((
            line.id,
            kind,
            encode_coordinates(&line.coordinates, COORDINATE_SCALE),
        ))?;
        insert_index.execute((line.id, bounds.0, bounds.1, bounds.2, bounds.3))?;
    }

    Ok(())
}
