mod mark_kept_points;
mod meters_per_degree_limit;
mod project_to_plane;

use mark_kept_points::mark_kept_points;
use project_to_plane::project_to_plane;

/// 度単位の（緯度, 経度）で表現された折れ線を Douglas–Peucker 法で簡略化し、元の折れ線からの距離がメートル単位の `tolerance` を超えない部分列を返す。距離は実距離を下回らない縮尺で評価するため、除かれた点の実距離の偏差は `tolerance` を超えない。両端の点は常に保持する。事前条件として、`tolerance` は正の値であり、折れ線は局所的な平面近似が成り立つ広がり（極付近や経度 180 度線をまたぐ折れ線を除く）に収まっていること。
#[must_use]
pub fn simplify_polyline(points: &[(f64, f64)], tolerance: f64) -> Vec<(f64, f64)> {
    if points.len() < 3 {
        return points.to_vec();
    }

    let projected = project_to_plane(points);
    let kept = mark_kept_points(&projected, tolerance);

    points
        .iter()
        .zip(&kept)
        .filter_map(|(&point, &is_kept)| is_kept.then_some(point))
        .collect()
}
