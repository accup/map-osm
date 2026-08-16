use crate::line::Line;
use crate::line_group::LineGroup;
use crate::point::Point;

/// データベースへ格納する内容の全体。
#[derive(Debug, Clone, PartialEq)]
pub struct DatabaseContent {
    /// キーと値の組で表現する付帯情報。
    pub metadata: Vec<(String, String)>,
    /// 折れ線のグループの列。
    pub line_groups: Vec<LineGroup>,
    /// 折れ線の列。
    pub lines: Vec<Line>,
    /// 点の列。
    pub points: Vec<Point>,
}
