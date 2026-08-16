use std::collections::HashSet;

use crate::line::Line;
use crate::line_group::LineGroup;

/// グループを入力の順序どおりに 1 から始まる識別子を与えて `line_group` テーブルへ挿入し、折れ線の列に存在する構成要素を `line_group_member` テーブルへ挿入する。重複する構成要素は 1 つに纏める。
pub(crate) fn insert_line_groups(
    transaction: &rusqlite::Transaction<'_>,
    line_groups: &[LineGroup],
    lines: &[Line],
) -> rusqlite::Result<()> {
    let line_ids: HashSet<i64> = lines.iter().map(|line| line.id).collect();

    let mut insert_group = transaction
        .prepare("INSERT INTO line_group (id, kind, reference, name) VALUES (?1, ?2, ?3, ?4)")?;
    let mut insert_member = transaction
        .prepare("INSERT OR IGNORE INTO line_group_member (group_id, line_id) VALUES (?1, ?2)")?;

    for (group_id, group) in (1_i64..).zip(line_groups) {
        insert_group.execute((group_id, group.kind, &group.reference, &group.name))?;

        for &line_id in &group.member_line_ids {
            if line_ids.contains(&line_id) {
                insert_member.execute((group_id, line_id))?;
            }
        }
    }

    Ok(())
}
