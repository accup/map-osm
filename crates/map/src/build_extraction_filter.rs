use osm_extract::ExtractionFilter;
use tag_classify::classify;

use crate::accepted_member_roles::ACCEPTED_MEMBER_ROLES;
use crate::line_classification_rules::line_classification_rules;
use crate::point_classification_rules::point_classification_rules;
use crate::relation_classification_rules::relation_classification_rules;

/// 分類規則の表のいずれかに合致する地点・路線・路線のリレーションを抽出し、形状の構成要素の役割のみを辿る選別条件を作る。
pub fn build_extraction_filter() -> ExtractionFilter {
    let point_rules = point_classification_rules();
    let line_rules = line_classification_rules();
    let relation_rules = relation_classification_rules();

    ExtractionFilter {
        point: Box::new(move |tags| classify(&point_rules, tags.pairs()).is_some()),
        line: Box::new(move |tags| classify(&line_rules, tags.pairs()).is_some()),
        relation: Box::new(move |tags| classify(&relation_rules, tags.pairs()).is_some()),
        member_role: Box::new(|role| ACCEPTED_MEMBER_ROLES.contains(&role)),
        include_relation_member_lines: true,
    }
}
