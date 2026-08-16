use tag_classify::{Condition, Matching, Rule};

use crate::point_kind::PointKind;

/// ノードのタグから地点の種別を判定する規則の表を返す。駅と停留場を駅として、高速道路の分岐点をインターチェンジとして判定する。
pub fn point_classification_rules() -> Vec<Rule<PointKind>> {
    vec![
        Rule::new(
            vec![Condition::new("railway", Matching::Exact, "station")],
            PointKind::Station,
        ),
        Rule::new(
            vec![Condition::new("railway", Matching::Exact, "halt")],
            PointKind::Station,
        ),
        Rule::new(
            vec![Condition::new(
                "highway",
                Matching::Exact,
                "motorway_junction",
            )],
            PointKind::Interchange,
        ),
    ]
}
