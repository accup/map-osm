use crate::extracted_relation::ExtractedRelation;

/// 関係の列に含まれるウェイの構成要素の識別子を、昇順かつ重複なしの列として返す。
pub(crate) fn relation_member_way_ids(relations: &[ExtractedRelation]) -> Vec<i64> {
    let mut way_ids: Vec<i64> = relations
        .iter()
        .flat_map(|relation| relation.member_way_ids.iter().copied())
        .collect();

    way_ids.sort_unstable();
    way_ids.dedup();

    way_ids
}
