use super::meters_per_degree_limit::METERS_PER_DEGREE_LIMIT;

/// 度単位の（緯度, 経度）の座標列を、局所的な平面近似でメートル単位の直交座標列へ射影する。縮尺には 1 度あたりの距離の上限値を用い、経度方向は座標列の緯度の絶対値の最小値の余弦で補正することで、射影上の距離が実距離を下回らないことを保証する。
pub(crate) fn project_to_plane(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let minimum_absolute_latitude = points
        .iter()
        .map(|point| point.0.abs())
        .fold(f64::INFINITY, f64::min);
    let longitude_scale = METERS_PER_DEGREE_LIMIT * minimum_absolute_latitude.to_radians().cos();

    points
        .iter()
        .map(|&(latitude, longitude)| {
            (
                latitude * METERS_PER_DEGREE_LIMIT,
                longitude * longitude_scale,
            )
        })
        .collect()
}
