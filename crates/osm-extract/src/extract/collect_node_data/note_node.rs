use crate::extracted_point::ExtractedPoint;
use crate::extraction_filter::ExtractionFilter;
use crate::tags::Tags;

use super::node_data::NodeData;

/// 1 つのノードから、昇順に整列されたノードの識別子の列に含まれる場合は座標の格納位置を、タグを持ち点の条件に合致する場合は点を収集する。
pub(crate) fn note_node<'a>(
    id: i64,
    latitude: f64,
    longitude: f64,
    tags: impl Iterator<Item = (&'a str, &'a str)>,
    filter: &ExtractionFilter,
    node_ids: &[i64],
) -> NodeData {
    let mut data = NodeData::default();

    if let Ok(index) = node_ids.binary_search(&id) {
        data.coordinate_slots.push((index, (latitude, longitude)));
    }

    let mut remaining = tags.peekable();
    if remaining.peek().is_some() {
        let tags: Tags = remaining.collect();
        if (filter.point)(&tags) {
            data.points.push(ExtractedPoint {
                id,
                tags,
                latitude,
                longitude,
            });
        }
    }

    data
}
