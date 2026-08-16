use crate::coordinate_scale::COORDINATE_SCALE;

/// 座標列の各成分に、格納時と同じ量子化（`COORDINATE_SCALE` 倍して最近接整数へ丸め、元の縮尺へ戻す）を適用した座標列を返す。
pub(crate) fn quantize_coordinates(coordinates: &[(f64, f64)]) -> Vec<(f64, f64)> {
    coordinates
        .iter()
        .map(|&(first, second)| {
            (
                (first * COORDINATE_SCALE).round() / COORDINATE_SCALE,
                (second * COORDINATE_SCALE).round() / COORDINATE_SCALE,
            )
        })
        .collect()
}
