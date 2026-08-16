use geo_sqlite::{DatabaseContent, Line, LineGroup, Point};
use osm_extract::Extraction;
use polyline_simplify::simplify_polyline;
use tag_classify::classify;

use crate::database_metadata::database_metadata;
use crate::line_classification_rules::line_classification_rules;
use crate::point_classification_rules::point_classification_rules;
use crate::relation_classification_rules::relation_classification_rules;
use crate::route_kind::RouteKind;
use crate::simplify_tolerance_meters::SIMPLIFY_TOLERANCE_METERS;

/// 抽出された地物を分類規則の表で種別へ分類し、形状を簡略化して、データベースへ格納する内容の全体を組み立てる。種別へ分類できない地点・関係は含めない。
pub fn build_database_content(extraction: &Extraction) -> DatabaseContent {
    let point_rules = point_classification_rules();
    let line_rules = line_classification_rules();
    let relation_rules = relation_classification_rules();

    let line_groups = extraction
        .relations
        .iter()
        .filter_map(|relation| {
            let kind = classify(&relation_rules, relation.tags.pairs())?;
            Some(LineGroup {
                kind: kind.code(),
                reference: relation.tags.value_of("ref").map(str::to_string),
                name: relation.tags.value_of("name").map(str::to_string),
                member_line_ids: relation.member_way_ids.clone(),
            })
        })
        .collect();

    let lines = extraction
        .lines
        .iter()
        .map(|line| Line {
            id: line.id,
            kind: classify(&line_rules, line.tags.pairs()).map(RouteKind::code),
            coordinates: simplify_polyline(&line.coordinates, SIMPLIFY_TOLERANCE_METERS),
        })
        .collect();

    let points = extraction
        .points
        .iter()
        .filter_map(|point| {
            let kind = classify(&point_rules, point.tags.pairs())?;
            Some(Point {
                id: point.id,
                kind: kind.code(),
                reference: point.tags.value_of("ref").map(str::to_string),
                name: point.tags.value_of("name").map(str::to_string),
                latitude: point.latitude,
                longitude: point.longitude,
            })
        })
        .collect();

    DatabaseContent {
        metadata: database_metadata(),
        line_groups,
        lines,
        points,
    }
}
