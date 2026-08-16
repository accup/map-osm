use crate::extracted_line::ExtractedLine;
use crate::extracted_point::ExtractedPoint;
use crate::extracted_relation::ExtractedRelation;

/// 抽出された地物の全体。各列は識別子の昇順に整列されている。
#[derive(Debug, Clone, PartialEq)]
pub struct Extraction {
    /// 抽出された点の列。
    pub points: Vec<ExtractedPoint>,
    /// 抽出された折れ線の列。
    pub lines: Vec<ExtractedLine>,
    /// 抽出された関係の列。
    pub relations: Vec<ExtractedRelation>,
}
