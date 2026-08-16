use polyline_simplify::simplify_polyline;

#[test]
fn returns_polylines_with_fewer_than_three_points_unchanged() {
    let two_points = [(35.0, 139.0), (35.1, 139.1)];

    assert_eq!(simplify_polyline(&[], 10.0), Vec::new());
    assert_eq!(simplify_polyline(&two_points, 10.0), two_points.to_vec());
}

#[test]
fn keeps_both_endpoints() {
    let points = [(35.0, 139.0), (35.05, 139.05), (35.1, 139.1)];

    let simplified = simplify_polyline(&points, 1.0);

    assert_eq!(simplified.first(), Some(&(35.0, 139.0)));
    assert_eq!(simplified.last(), Some(&(35.1, 139.1)));
}

#[test]
fn removes_collinear_intermediate_points() {
    let points = [
        (35.0, 139.0),
        (35.025, 139.0),
        (35.05, 139.0),
        (35.075, 139.0),
        (35.1, 139.0),
    ];

    let simplified = simplify_polyline(&points, 0.001);

    assert_eq!(simplified, vec![(35.0, 139.0), (35.1, 139.0)]);
}

#[test]
fn applies_tolerance_in_meters_to_latitude_deviation() {
    // 中央の点は両端を結ぶ線分から緯度方向へ 0.00045 度（約 50 メートル）離れている。
    let points = [(35.0, 139.0), (35.000_45, 139.005), (35.0, 139.01)];

    assert_eq!(simplify_polyline(&points, 100.0).len(), 2);
    assert_eq!(simplify_polyline(&points, 10.0).len(), 3);
}

#[test]
fn applies_latitude_correction_to_longitude_deviation() {
    // 中央の点は経度方向へ 0.001 度離れており、緯度 60 度では約 55.7 メートルに相当する。
    let points = [(59.995, 10.0), (60.0, 10.001), (60.005, 10.0)];

    assert_eq!(simplify_polyline(&points, 60.0).len(), 2);
    assert_eq!(simplify_polyline(&points, 50.0).len(), 3);
}

#[test]
fn applies_the_upper_bound_scale_to_latitude_distances() {
    // 中央の点は緯度方向へ 0.001 度離れている。1 度あたり距離の上限値による評価では約 111.69 メートル、赤道円周由来の縮尺による評価では約 111.32 メートルになるため、許容誤差をその間に置き、距離が上限値の縮尺で評価されることを検証する。
    let points = [(35.0, 139.0), (35.001, 139.05), (35.0, 139.1)];

    assert_eq!(simplify_polyline(&points, 111.5).len(), 3);
    assert_eq!(simplify_polyline(&points, 111.8).len(), 2);
}

#[test]
fn applies_the_upper_bound_scale_to_longitude_distances() {
    // 中央の点は経度方向へ 0.001 度離れている。上限値と緯度の絶対値の最小値（35 度）の余弦による評価では約 91.49 メートル、赤道円周由来の縮尺と緯度の平均値の余弦による評価では約 91.13 メートルになるため、許容誤差をその間に置く。
    let points = [(35.0, 139.0), (35.05, 139.001), (35.1, 139.0)];

    assert_eq!(simplify_polyline(&points, 91.3).len(), 3);
    assert_eq!(simplify_polyline(&points, 91.7).len(), 2);
}

#[test]
fn keeps_all_turning_points_exceeding_tolerance() {
    let points = [
        (35.0, 139.0),
        (35.01, 139.01),
        (35.005, 139.02),
        (34.99, 139.03),
        (35.0, 139.04),
    ];

    let simplified = simplify_polyline(&points, 10.0);

    assert_eq!(simplified, points.to_vec());
}
