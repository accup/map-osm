mod node_data;
mod note_node;

use std::path::Path;

use osmpbf::{Element, ElementReader};

use crate::extract_error::ExtractError;
use crate::extraction_filter::ExtractionFilter;
use node_data::NodeData;
use note_node::note_node;

use super::resolved_nodes::ResolvedNodes;

/// ファイル内のすべてのノードから、昇順に整列されたノードの識別子の列に対応する座標の列（存在しないノードは `None`）と、点の条件に合致するタグを持つノードの点の列を収集する。
pub(crate) fn collect_node_data(
    path: &Path,
    filter: &ExtractionFilter,
    node_ids: &[i64],
) -> Result<ResolvedNodes, ExtractError> {
    let reader = ElementReader::from_path(path).map_err(ExtractError::new)?;

    let data = reader
        .par_map_reduce(
            |element| match element {
                Element::Node(node) => note_node(
                    node.id(),
                    node.lat(),
                    node.lon(),
                    node.tags(),
                    filter,
                    node_ids,
                ),
                Element::DenseNode(node) => note_node(
                    node.id(),
                    node.lat(),
                    node.lon(),
                    node.tags(),
                    filter,
                    node_ids,
                ),
                _ => NodeData::default(),
            },
            NodeData::default,
            NodeData::merge,
        )
        .map_err(ExtractError::new)?;

    let mut coordinates = vec![None; node_ids.len()];
    for (index, coordinate) in data.coordinate_slots {
        coordinates[index] = Some(coordinate);
    }

    Ok(ResolvedNodes {
        coordinates,
        points: data.points,
    })
}
