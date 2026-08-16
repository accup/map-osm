use crate::extracted_point::ExtractedPoint;

/// ノードの走査で収集する、座標の格納位置と点の集まり。
#[derive(Debug, Default)]
pub(crate) struct NodeData {
    /// 昇順に整列されたノードの識別子の列における位置と、そのノードの座標の組の列。
    pub(crate) coordinate_slots: Vec<(usize, (f64, f64))>,
    /// 点の条件に合致したノードの点の列。
    pub(crate) points: Vec<ExtractedPoint>,
}

impl NodeData {
    /// 2 つの集まりを 1 つに結合する。
    #[must_use]
    pub(crate) fn merge(mut left: Self, mut right: Self) -> Self {
        left.coordinate_slots.append(&mut right.coordinate_slots);
        left.points.append(&mut right.points);
        left
    }
}
