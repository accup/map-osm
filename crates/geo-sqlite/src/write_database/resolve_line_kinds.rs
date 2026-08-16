use std::collections::HashMap;

use crate::geo_sqlite_error::GeoSqliteError;
use crate::line::Line;
use crate::line_group::LineGroup;

/// 各折れ線の種別を、折れ線自身の種別、またはその折れ線を構成要素に含む最初のグループの種別として解決し、折れ線の列と同じ順序で返す。
///
/// # Errors
///
/// 種別を持たない折れ線が種別を持つグループに属していない場合、エラーを返す。
pub(crate) fn resolve_line_kinds(
    lines: &[Line],
    line_groups: &[LineGroup],
) -> Result<Vec<i64>, GeoSqliteError> {
    let mut inherited_kinds: HashMap<i64, i64> = HashMap::new();
    for group in line_groups {
        for &line_id in &group.member_line_ids {
            inherited_kinds.entry(line_id).or_insert(group.kind);
        }
    }

    lines
        .iter()
        .map(|line| {
            line.kind
                .or_else(|| inherited_kinds.get(&line.id).copied())
                .ok_or(GeoSqliteError::UnresolvedLineKind(line.id))
        })
        .collect()
}
