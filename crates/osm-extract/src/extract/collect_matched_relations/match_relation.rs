use osmpbf::RelMemberType;

use crate::extracted_relation::ExtractedRelation;
use crate::extraction_filter::ExtractionFilter;
use crate::tags::Tags;

/// リレーションが関係の条件に合致する場合、役割の条件を満たすウェイの構成要素の識別子とともに関係として返す。
///
/// # Errors
///
/// 構成要素の役割を読み取れない場合、エラーを返す。
pub(crate) fn match_relation(
    relation: &osmpbf::elements::Relation<'_>,
    filter: &ExtractionFilter,
) -> Result<Option<ExtractedRelation>, osmpbf::Error> {
    let tags: Tags = relation.tags().collect();
    if !(filter.relation)(&tags) {
        return Ok(None);
    }

    let mut member_way_ids = Vec::new();
    for member in relation.members() {
        if member.member_type == RelMemberType::Way && (filter.member_role)(member.role()?) {
            member_way_ids.push(member.member_id);
        }
    }

    Ok(Some(ExtractedRelation {
        id: relation.id(),
        tags,
        member_way_ids,
    }))
}
