mod match_relation;

use std::path::Path;

use osmpbf::{Element, ElementReader};

use crate::extract_error::ExtractError;
use crate::extracted_relation::ExtractedRelation;
use crate::extraction_filter::ExtractionFilter;
use match_relation::match_relation;

/// ファイル内のすべてのリレーションのうち、関係の条件に合致するものを関係として収集する。
pub(crate) fn collect_matched_relations(
    path: &Path,
    filter: &ExtractionFilter,
) -> Result<Vec<ExtractedRelation>, ExtractError> {
    let reader = ElementReader::from_path(path).map_err(ExtractError::new)?;

    let relations = reader
        .par_map_reduce(
            |element| match element {
                Element::Relation(relation) => {
                    match_relation(&relation, filter).map(|matched| matched.into_iter().collect())
                }
                _ => Ok(Vec::new()),
            },
            || Ok(Vec::new()),
            |left, right| {
                let (mut left, mut right) = (left?, right?);
                left.append(&mut right);
                Ok(left)
            },
        )
        .map_err(ExtractError::new)?
        .map_err(ExtractError::new)?;

    Ok(relations)
}
