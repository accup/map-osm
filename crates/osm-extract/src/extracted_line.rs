use crate::tags::Tags;

/// 抽出された折れ線。ノードの座標を解決したウェイに対応する。
#[derive(Debug, Clone, PartialEq)]
pub struct ExtractedLine {
    /// ウェイの識別子。
    pub id: i64,
    /// ウェイのタグ。
    pub tags: Tags,
    /// 度単位の（緯度, 経度）で表現する座標列。
    pub coordinates: Vec<(f64, f64)>,
}
