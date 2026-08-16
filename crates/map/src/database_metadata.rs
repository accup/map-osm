use crate::japan_osm_pbf_url::JAPAN_OSM_PBF_URL;

/// データベースへ格納する付帯情報を返す。出典の表示・ライセンス・取得元と、種別の値の意味を含む。
pub fn database_metadata() -> Vec<(String, String)> {
    [
        (
            "attribution",
            "© OpenStreetMap contributors (https://www.openstreetmap.org/copyright)",
        ),
        (
            "license",
            "ODbL 1.0 (https://opendatacommons.org/licenses/odbl/1-0/)",
        ),
        ("source", JAPAN_OSM_PBF_URL),
        ("line_kind.1", "railway"),
        ("line_kind.2", "expressway"),
        ("line_kind.3", "national_road"),
        ("line_kind.4", "prefectural_road"),
        ("point_kind.1", "station"),
        ("point_kind.2", "interchange"),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value.to_string()))
    .collect()
}
