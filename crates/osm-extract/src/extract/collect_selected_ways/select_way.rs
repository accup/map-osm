use crate::extraction_filter::ExtractionFilter;
use crate::tags::Tags;

use super::super::selected_way::SelectedWay;

/// ウェイが折れ線の条件に合致するか、昇順に整列された構成要素の識別子の列に含まれる場合、参照するノードの識別子とともに選ばれたウェイとして返す。
pub(crate) fn select_way(
    way: &osmpbf::elements::Way<'_>,
    filter: &ExtractionFilter,
    member_way_ids: &[i64],
) -> Option<SelectedWay> {
    let tags: Tags = way.tags().collect();
    let is_member = member_way_ids.binary_search(&way.id()).is_ok();
    if !is_member && !(filter.line)(&tags) {
        return None;
    }

    Some(SelectedWay {
        id: way.id(),
        tags,
        node_ids: way.refs().collect(),
    })
}
