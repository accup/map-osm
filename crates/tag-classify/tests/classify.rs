use tag_classify::{Condition, Matching, Rule, classify};

fn tags(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
    pairs
        .iter()
        .map(|&(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

#[test]
fn classifies_by_exact_value_match() {
    let rules = vec![Rule::new(
        vec![Condition::new("highway", Matching::Exact, "motorway")],
        "expressway",
    )];

    assert_eq!(
        classify(&rules, &tags(&[("highway", "motorway")])),
        Some("expressway")
    );
    assert_eq!(
        classify(&rules, &tags(&[("highway", "motorway_link")])),
        None
    );
}

#[test]
fn classifies_by_prefix_value_match() {
    let rules = vec![Rule::new(
        vec![Condition::new(
            "network",
            Matching::Prefix,
            "JP:prefectural",
        )],
        "prefectural",
    )];

    assert_eq!(
        classify(&rules, &tags(&[("network", "JP:prefectural:aichi")])),
        Some("prefectural")
    );
    assert_eq!(classify(&rules, &tags(&[("network", "JP:national")])), None);
}

#[test]
fn requires_all_conditions_of_a_rule() {
    let rules = vec![Rule::new(
        vec![
            Condition::new("type", Matching::Exact, "route"),
            Condition::new("route", Matching::Exact, "train"),
        ],
        "railway",
    )];

    assert_eq!(
        classify(&rules, &tags(&[("type", "route"), ("route", "train")])),
        Some("railway")
    );
    assert_eq!(classify(&rules, &tags(&[("route", "train")])), None);
}

#[test]
fn returns_the_label_of_the_first_satisfied_rule() {
    let rules = vec![
        Rule::new(
            vec![Condition::new(
                "network",
                Matching::Prefix,
                "JP:national:expressway",
            )],
            "expressway",
        ),
        Rule::new(
            vec![Condition::new("network", Matching::Prefix, "JP:national")],
            "national",
        ),
    ];

    assert_eq!(
        classify(&rules, &tags(&[("network", "JP:national:expressway")])),
        Some("expressway")
    );
    assert_eq!(
        classify(&rules, &tags(&[("network", "JP:national")])),
        Some("national")
    );
}

#[test]
fn ignores_values_of_other_keys() {
    let rules = vec![Rule::new(
        vec![Condition::new("railway", Matching::Exact, "rail")],
        "railway",
    )];

    assert_eq!(classify(&rules, &tags(&[("highway", "rail")])), None);
}

#[test]
fn matches_any_pair_having_the_same_key() {
    let rules = vec![Rule::new(
        vec![Condition::new("route", Matching::Exact, "train")],
        "railway",
    )];

    assert_eq!(
        classify(&rules, &tags(&[("route", "bus"), ("route", "train")])),
        Some("railway")
    );
}
