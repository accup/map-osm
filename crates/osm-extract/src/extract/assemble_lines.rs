use crate::extracted_line::ExtractedLine;

use super::selected_way::SelectedWay;

/// 選ばれたウェイのノードの参照を、昇順に整列されたノードの識別子の列とそれに対応する座標の列で解決し、折れ線の列を組み立てる。解決できない参照は取り除き、解決後の座標が 2 点に満たないウェイは折れ線としない。
pub(crate) fn assemble_lines(
    ways: Vec<SelectedWay>,
    node_ids: &[i64],
    coordinates: &[Option<(f64, f64)>],
) -> Vec<ExtractedLine> {
    ways.into_iter()
        .filter_map(|way| {
            let resolved: Vec<(f64, f64)> = way
                .node_ids
                .iter()
                .filter_map(|node_id| {
                    let index = node_ids.binary_search(node_id).ok()?;
                    coordinates[index]
                })
                .collect();

            (resolved.len() >= 2).then_some(ExtractedLine {
                id: way.id,
                tags: way.tags,
                coordinates: resolved,
            })
        })
        .collect()
}
