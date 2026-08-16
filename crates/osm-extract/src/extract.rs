mod assemble_lines;
mod collect_matched_relations;
mod collect_node_data;
mod collect_selected_ways;
mod relation_member_way_ids;
mod resolved_nodes;
mod selected_way;
mod way_node_ids;

use std::path::Path;

use crate::extract_error::ExtractError;
use crate::extraction::Extraction;
use crate::extraction_filter::ExtractionFilter;
use assemble_lines::assemble_lines;
use collect_matched_relations::collect_matched_relations;
use collect_node_data::collect_node_data;
use collect_selected_ways::collect_selected_ways;
use relation_member_way_ids::relation_member_way_ids;
use way_node_ids::way_node_ids;

/// OSM PBF ファイルから、選別条件に合致する点・折れ線・関係を抽出する。折れ線の座標はファイル内のノードから解決し、解決できないノードの参照は取り除く。解決後の座標が 2 点に満たないウェイは折れ線として抽出しない。各列は識別子の昇順で返す。
///
/// # Errors
///
/// ファイルを開けない場合、または OSM PBF 形式として読み取れない場合、エラーを返す。
pub fn extract(path: &Path, filter: &ExtractionFilter) -> Result<Extraction, ExtractError> {
    let mut relations = collect_matched_relations(path, filter)?;

    let member_way_ids = if filter.include_relation_member_lines {
        relation_member_way_ids(&relations)
    } else {
        Vec::new()
    };
    let ways = collect_selected_ways(path, filter, &member_way_ids)?;

    let node_ids = way_node_ids(&ways);
    let resolved = collect_node_data(path, filter, &node_ids)?;

    let mut lines = assemble_lines(ways, &node_ids, &resolved.coordinates);

    let mut points = resolved.points;
    points.sort_unstable_by_key(|point| point.id);
    lines.sort_unstable_by_key(|line| line.id);
    relations.sort_unstable_by_key(|relation| relation.id);

    Ok(Extraction {
        points,
        lines,
        relations,
    })
}
