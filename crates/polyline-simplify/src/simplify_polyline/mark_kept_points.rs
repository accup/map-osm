mod point_to_segment_distance;

use point_to_segment_distance::point_to_segment_distance;

/// 平面座標列に対して Douglas–Peucker 法を適用し、保持する点を真とする列を返す。両端の点は常に真とし、区間の両端を結ぶ線分からの距離が `tolerance` を超える点が存在する区間を再帰的に分割する。事前条件として、座標列は 3 点以上であること。
pub(crate) fn mark_kept_points(points: &[(f64, f64)], tolerance: f64) -> Vec<bool> {
    let mut kept = vec![false; points.len()];
    kept[0] = true;
    kept[points.len() - 1] = true;

    let mut sections = vec![(0_usize, points.len() - 1)];
    while let Some((start, end)) = sections.pop() {
        if end <= start + 1 {
            continue;
        }

        let mut farthest_distance = 0.0_f64;
        let mut farthest_index = start;
        for (offset, &point) in points[start + 1..end].iter().enumerate() {
            let distance = point_to_segment_distance(point, points[start], points[end]);
            if distance > farthest_distance {
                farthest_distance = distance;
                farthest_index = start + 1 + offset;
            }
        }

        if farthest_distance > tolerance {
            kept[farthest_index] = true;
            sections.push((start, farthest_index));
            sections.push((farthest_index, end));
        }
    }

    kept
}
