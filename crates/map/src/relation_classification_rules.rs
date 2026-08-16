use tag_classify::{Condition, Matching, Rule};

use crate::route_kind::RouteKind;

/// リレーションのタグから路線の種別を判定する規則の表を返す。鉄道は経路の種類で、道路は `network` タグで判定する。`network` タグの表記揺れ（都道府県名の接尾辞・誤記）は前方一致の規則で吸収し、高速道路の規則を国道の規則より先に評価する。
pub fn relation_classification_rules() -> Vec<Rule<RouteKind>> {
    let railway_route_values = ["train", "subway", "tram", "monorail", "light_rail"];
    let expressway_networks = [
        (Matching::Prefix, "JP:national:expressway"),
        (Matching::Prefix, "JP:expressway"),
        (Matching::Exact, "JP:E"),
        (Matching::Exact, "首都高速道路"),
        (Matching::Exact, "名古屋高速道路"),
    ];
    let national_road_networks = [(Matching::Prefix, "JP:national")];
    let prefectural_road_networks = [
        (Matching::Prefix, "JP:prefectural"),
        (Matching::Prefix, "JP:perfectural"),
        (Matching::Prefix, "JP:prefecural"),
        (Matching::Exact, "主要地方道"),
        (Matching::Exact, "一般県道"),
        (Matching::Prefix, "北海道道"),
    ];

    let railway_rules = railway_route_values.into_iter().map(|value| {
        Rule::new(
            vec![
                Condition::new("type", Matching::Exact, "route"),
                Condition::new("route", Matching::Exact, value),
            ],
            RouteKind::Railway,
        )
    });
    let road_rules = expressway_networks
        .into_iter()
        .map(|network| (network, RouteKind::Expressway))
        .chain(
            national_road_networks
                .into_iter()
                .map(|network| (network, RouteKind::NationalRoad)),
        )
        .chain(
            prefectural_road_networks
                .into_iter()
                .map(|network| (network, RouteKind::PrefecturalRoad)),
        )
        .map(|((matching, pattern), kind)| {
            Rule::new(
                vec![
                    Condition::new("type", Matching::Exact, "route"),
                    Condition::new("route", Matching::Exact, "road"),
                    Condition::new("network", matching, pattern),
                ],
                kind,
            )
        });

    railway_rules.chain(road_rules).collect()
}
