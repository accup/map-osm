use crate::extracted_point::ExtractedPoint;

/// ノードの走査の結果。座標の列と点の列を持つ。
#[derive(Debug)]
pub(crate) struct ResolvedNodes {
    /// 昇順に整列されたノードの識別子の列に対応する座標の列。ファイル内に存在しないノードは `None` とする。
    pub(crate) coordinates: Vec<Option<(f64, f64)>>,
    /// 点の条件に合致したノードの点の列。
    pub(crate) points: Vec<ExtractedPoint>,
}
