use crate::tags::Tags;

/// 抽出された点。タグを持つノードに対応する。
#[derive(Debug, Clone, PartialEq)]
pub struct ExtractedPoint {
    /// ノードの識別子。
    pub id: i64,
    /// ノードのタグ。
    pub tags: Tags,
    /// 度単位の緯度。
    pub latitude: f64,
    /// 度単位の経度。
    pub longitude: f64,
}
