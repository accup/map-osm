use crate::tags::Tags;

/// 折れ線として抽出する対象に選ばれた、ノードの参照を解決する前のウェイ。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SelectedWay {
    /// ウェイの識別子。
    pub(crate) id: i64,
    /// ウェイのタグ。
    pub(crate) tags: Tags,
    /// ウェイが参照するノードの識別子の列。
    pub(crate) node_ids: Vec<i64>,
}
