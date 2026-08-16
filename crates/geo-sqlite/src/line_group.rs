/// 付番と名称を持つ折れ線のグループ。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineGroup {
    /// グループの種別。
    pub kind: i64,
    /// グループの付番。
    pub reference: Option<String>,
    /// グループの名称。
    pub name: Option<String>,
    /// グループに属する折れ線の識別子の列。
    pub member_line_ids: Vec<i64>,
}
