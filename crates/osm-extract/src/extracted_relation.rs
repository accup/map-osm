use crate::tags::Tags;

/// 抽出された関係。リレーションに対応する。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractedRelation {
    /// リレーションの識別子。
    pub id: i64,
    /// リレーションのタグ。
    pub tags: Tags,
    /// 役割の条件を満たすウェイの構成要素の識別子の列。リレーション内の順序を保持する。
    pub member_way_ids: Vec<i64>,
}
