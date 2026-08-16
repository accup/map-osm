/// 座標列の外接矩形を（第 1 成分の最小値, 第 1 成分の最大値, 第 2 成分の最小値, 第 2 成分の最大値）として返す。座標列が空の場合は `None` を返す。
pub(crate) fn coordinate_bounds(coordinates: &[(f64, f64)]) -> Option<(f64, f64, f64, f64)> {
    let (&first, rest) = coordinates.split_first()?;

    let mut bounds = (first.0, first.0, first.1, first.1);
    for &(latitude, longitude) in rest {
        bounds.0 = bounds.0.min(latitude);
        bounds.1 = bounds.1.max(latitude);
        bounds.2 = bounds.2.min(longitude);
        bounds.3 = bounds.3.max(longitude);
    }

    Some(bounds)
}
