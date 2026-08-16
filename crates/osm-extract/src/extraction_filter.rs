use crate::tags::Tags;

/// 抽出する地物の選別条件。
pub struct ExtractionFilter {
    /// タグを持つノードを点として抽出するかを、タグから判定する述語。
    pub point: Box<dyn Fn(&Tags) -> bool + Send + Sync>,
    /// ウェイを折れ線として抽出するかを、タグから判定する述語。
    pub line: Box<dyn Fn(&Tags) -> bool + Send + Sync>,
    /// リレーションを関係として抽出するかを、タグから判定する述語。
    pub relation: Box<dyn Fn(&Tags) -> bool + Send + Sync>,
    /// リレーションのウェイの構成要素を関係に含めるかを、構成要素の役割から判定する述語。
    pub member_role: Box<dyn Fn(&str) -> bool + Send + Sync>,
    /// 抽出された関係の構成要素のウェイを、折れ線の条件に合致しない場合も折れ線として抽出するか。
    pub include_relation_member_lines: bool,
}
