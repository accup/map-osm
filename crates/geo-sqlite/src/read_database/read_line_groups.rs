use std::collections::HashMap;

use crate::line_group::LineGroup;

/// `line_group` テーブルと `line_group_member` テーブルから、構成要素の識別子の列を含むグループの列を識別子の昇順で読み出す。
pub(crate) fn read_line_groups(
    connection: &rusqlite::Connection,
) -> rusqlite::Result<Vec<LineGroup>> {
    let mut select_members =
        connection.prepare("SELECT group_id, line_id FROM line_group_member ORDER BY rowid")?;
    let mut members_by_group: HashMap<i64, Vec<i64>> = HashMap::new();
    for member in select_members.query_map((), |row| Ok((row.get::<_, i64>(0)?, row.get(1)?)))? {
        let (group_id, line_id) = member?;
        members_by_group.entry(group_id).or_default().push(line_id);
    }

    let mut select_groups =
        connection.prepare("SELECT id, kind, reference, name FROM line_group ORDER BY id")?;
    let line_groups = select_groups
        .query_map((), |row| {
            Ok(LineGroup {
                kind: row.get(1)?,
                reference: row.get(2)?,
                name: row.get(3)?,
                member_line_ids: members_by_group
                    .get(&row.get::<_, i64>(0)?)
                    .cloned()
                    .unwrap_or_default(),
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(line_groups)
}
