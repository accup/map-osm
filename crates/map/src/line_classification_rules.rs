use tag_classify::{Condition, Matching, Rule};

use crate::route_kind::RouteKind;

/// ウェイのタグから路線の種別を判定する規則の表を返す。鉄道の線路と高速道路の本線・連絡路をウェイ単体で判定する。国道・都道府県道はウェイ単体では判定せず、リレーションの規則で判定する。
pub fn line_classification_rules() -> Vec<Rule<RouteKind>> {
    let railway_values = [
        "rail",
        "light_rail",
        "subway",
        "tram",
        "monorail",
        "narrow_gauge",
        "funicular",
    ];
    let expressway_values = ["motorway", "motorway_link"];

    let railway_rules = railway_values.into_iter().map(|value| {
        Rule::new(
            vec![Condition::new("railway", Matching::Exact, value)],
            RouteKind::Railway,
        )
    });
    let expressway_rules = expressway_values.into_iter().map(|value| {
        Rule::new(
            vec![Condition::new("highway", Matching::Exact, value)],
            RouteKind::Expressway,
        )
    });

    railway_rules.chain(expressway_rules).collect()
}
