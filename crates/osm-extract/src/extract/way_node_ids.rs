use super::selected_way::SelectedWay;

/// 選ばれたウェイの列が参照するノードの識別子を、昇順かつ重複なしの列として返す。
pub(crate) fn way_node_ids(ways: &[SelectedWay]) -> Vec<i64> {
    let mut node_ids: Vec<i64> = ways
        .iter()
        .flat_map(|way| way.node_ids.iter().copied())
        .collect();

    node_ids.sort_unstable();
    node_ids.dedup();

    node_ids
}
