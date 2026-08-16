/// 平面上の点から、始点と終点を結ぶ線分への最短距離を返す。始点と終点が一致する場合は点どうしの距離を返す。
pub(crate) fn point_to_segment_distance(
    point: (f64, f64),
    start: (f64, f64),
    end: (f64, f64),
) -> f64 {
    let segment = (end.0 - start.0, end.1 - start.1);
    let squared_length = segment.0 * segment.0 + segment.1 * segment.1;

    let offset = (point.0 - start.0, point.1 - start.1);
    let position = if squared_length == 0.0 {
        0.0
    } else {
        (offset.0 * segment.0 + offset.1 * segment.1) / squared_length
    };
    let clamped = position.clamp(0.0, 1.0);

    let nearest = (start.0 + clamped * segment.0, start.1 + clamped * segment.1);
    ((point.0 - nearest.0).powi(2) + (point.1 - nearest.1).powi(2)).sqrt()
}
