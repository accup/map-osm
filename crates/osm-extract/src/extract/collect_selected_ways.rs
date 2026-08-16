mod select_way;

use std::path::Path;

use osmpbf::{Element, ElementReader};

use crate::extract_error::ExtractError;
use crate::extraction_filter::ExtractionFilter;
use select_way::select_way;

use super::selected_way::SelectedWay;

/// ファイル内のすべてのウェイのうち、折れ線の条件に合致するもの、または昇順に整列された構成要素の識別子の列に含まれるものを収集する。
pub(crate) fn collect_selected_ways(
    path: &Path,
    filter: &ExtractionFilter,
    member_way_ids: &[i64],
) -> Result<Vec<SelectedWay>, ExtractError> {
    let reader = ElementReader::from_path(path).map_err(ExtractError::new)?;

    let ways = reader
        .par_map_reduce(
            |element| match element {
                Element::Way(way) => select_way(&way, filter, member_way_ids)
                    .into_iter()
                    .collect(),
                _ => Vec::new(),
            },
            Vec::new,
            |mut left, mut right| {
                left.append(&mut right);
                left
            },
        )
        .map_err(ExtractError::new)?;

    Ok(ways)
}
