use super::meters_per_degree::METERS_PER_DEGREE;

/// 度単位の（緯度, 経度）の座標列を、緯度の平均値における局所的な平面近似でメートル単位の直交座標列へ射影する。
#[allow(
    clippy::cast_precision_loss,
    reason = "点数は f64 の仮数部で表現できる大きさである"
)]
pub(crate) fn project_to_plane(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mean_latitude = points.iter().map(|point| point.0).sum::<f64>() / points.len() as f64;
    let longitude_scale = METERS_PER_DEGREE * mean_latitude.to_radians().cos();

    points
        .iter()
        .map(|&(latitude, longitude)| (latitude * METERS_PER_DEGREE, longitude * longitude_scale))
        .collect()
}
