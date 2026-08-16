use std::path::PathBuf;

use osm_extract::{ExtractionFilter, Tags, extract};

struct FixtureNode {
    id: i64,
    latitude: f64,
    longitude: f64,
    tags: Vec<(&'static str, &'static str)>,
}

struct FixtureWay {
    id: i64,
    tags: Vec<(&'static str, &'static str)>,
    node_ids: Vec<i64>,
}

struct FixtureRelation {
    id: i64,
    tags: Vec<(&'static str, &'static str)>,
    way_members: Vec<(&'static str, i64)>,
}

fn append_varint(mut value: u64, bytes: &mut Vec<u8>) {
    while value >= 0x80 {
        bytes.push(u8::try_from(value & 0x7f).unwrap() | 0x80);
        value >>= 7;
    }
    bytes.push(u8::try_from(value).unwrap());
}

#[allow(
    clippy::cast_sign_loss,
    reason = "zigzag 符号化は符号ビットを最下位ビットへ移すため、ビット表現の再解釈が意図した動作である"
)]
fn zigzag(value: i64) -> u64 {
    ((value << 1) ^ (value >> 63)) as u64
}

fn append_field_header(tag: u32, wire: u32, bytes: &mut Vec<u8>) {
    append_varint(u64::from((tag << 3) | wire), bytes);
}

fn append_bytes_field(tag: u32, data: &[u8], bytes: &mut Vec<u8>) {
    append_field_header(tag, 2, bytes);
    append_varint(data.len() as u64, bytes);
    bytes.extend_from_slice(data);
}

fn packed(values: impl IntoIterator<Item = u64>) -> Vec<u8> {
    let mut bytes = Vec::new();
    for value in values {
        append_varint(value, &mut bytes);
    }
    bytes
}

fn intern(strings: &mut Vec<String>, value: &str) -> u64 {
    if let Some(index) = strings.iter().position(|existing| existing == value) {
        return index as u64;
    }
    strings.push(value.to_string());
    strings.len() as u64 - 1
}

fn tag_indices(strings: &mut Vec<String>, tags: &[(&str, &str)]) -> (Vec<u64>, Vec<u64>) {
    let keys = tags.iter().map(|(key, _)| intern(strings, key)).collect();
    let values = tags
        .iter()
        .map(|(_, value)| intern(strings, value))
        .collect();
    (keys, values)
}

#[allow(
    clippy::cast_possible_truncation,
    reason = "経緯度の値域では 100 ナノ度単位の値が i64 に収まる"
)]
fn coordinate_unit(degrees: f64) -> i64 {
    // granularity の既定値 100 ナノ度を単位とする。
    (degrees * 1e7).round() as i64
}

fn append_plain_node_group(strings: &mut Vec<String>, nodes: &[FixtureNode], block: &mut Vec<u8>) {
    let mut group = Vec::new();
    for node in nodes {
        let (keys, values) = tag_indices(strings, &node.tags);
        let mut message = Vec::new();
        append_field_header(1, 0, &mut message);
        append_varint(zigzag(node.id), &mut message);
        append_bytes_field(2, &packed(keys), &mut message);
        append_bytes_field(3, &packed(values), &mut message);
        append_field_header(8, 0, &mut message);
        append_varint(zigzag(coordinate_unit(node.latitude)), &mut message);
        append_field_header(9, 0, &mut message);
        append_varint(zigzag(coordinate_unit(node.longitude)), &mut message);
        append_bytes_field(1, &message, &mut group);
    }
    append_bytes_field(2, &group, block);
}

fn append_dense_node_group(strings: &mut Vec<String>, nodes: &[FixtureNode], block: &mut Vec<u8>) {
    let mut ids = Vec::new();
    let mut latitudes = Vec::new();
    let mut longitudes = Vec::new();
    let mut keys_values = Vec::new();
    let mut previous = (0_i64, 0_i64, 0_i64);
    for node in nodes {
        ids.push(zigzag(node.id - previous.0));
        latitudes.push(zigzag(coordinate_unit(node.latitude) - previous.1));
        longitudes.push(zigzag(coordinate_unit(node.longitude) - previous.2));
        previous = (
            node.id,
            coordinate_unit(node.latitude),
            coordinate_unit(node.longitude),
        );
        for (key, value) in &node.tags {
            keys_values.push(intern(strings, key));
            keys_values.push(intern(strings, value));
        }
        keys_values.push(0);
    }

    let mut dense = Vec::new();
    append_bytes_field(1, &packed(ids), &mut dense);
    append_bytes_field(8, &packed(latitudes), &mut dense);
    append_bytes_field(9, &packed(longitudes), &mut dense);
    append_bytes_field(10, &packed(keys_values), &mut dense);

    let mut group = Vec::new();
    append_bytes_field(2, &dense, &mut group);
    append_bytes_field(2, &group, block);
}

fn append_way_group(strings: &mut Vec<String>, ways: &[FixtureWay], block: &mut Vec<u8>) {
    let mut group = Vec::new();
    for way in ways {
        let (keys, values) = tag_indices(strings, &way.tags);
        let mut references = Vec::new();
        let mut previous = 0_i64;
        for &node_id in &way.node_ids {
            references.push(zigzag(node_id - previous));
            previous = node_id;
        }

        let mut message = Vec::new();
        append_field_header(1, 0, &mut message);
        append_varint(u64::try_from(way.id).unwrap(), &mut message);
        append_bytes_field(2, &packed(keys), &mut message);
        append_bytes_field(3, &packed(values), &mut message);
        append_bytes_field(8, &packed(references), &mut message);
        append_bytes_field(3, &message, &mut group);
    }
    append_bytes_field(2, &group, block);
}

fn append_relation_group(
    strings: &mut Vec<String>,
    relations: &[FixtureRelation],
    block: &mut Vec<u8>,
) {
    let mut group = Vec::new();
    for relation in relations {
        let (keys, values) = tag_indices(strings, &relation.tags);
        let roles: Vec<u64> = relation
            .way_members
            .iter()
            .map(|(role, _)| intern(strings, role))
            .collect();
        let mut member_ids = Vec::new();
        let mut previous = 0_i64;
        for &(_, way_id) in &relation.way_members {
            member_ids.push(zigzag(way_id - previous));
            previous = way_id;
        }
        let types = vec![1_u64; relation.way_members.len()];

        let mut message = Vec::new();
        append_field_header(1, 0, &mut message);
        append_varint(u64::try_from(relation.id).unwrap(), &mut message);
        append_bytes_field(2, &packed(keys), &mut message);
        append_bytes_field(3, &packed(values), &mut message);
        append_bytes_field(8, &packed(roles), &mut message);
        append_bytes_field(9, &packed(member_ids), &mut message);
        append_bytes_field(10, &packed(types), &mut message);
        append_bytes_field(4, &message, &mut group);
    }
    append_bytes_field(2, &group, block);
}

fn append_blob(kind: &str, content: &[u8], file: &mut Vec<u8>) {
    let mut header = Vec::new();
    append_bytes_field(1, kind.as_bytes(), &mut header);

    let mut body = Vec::new();
    append_bytes_field(1, content, &mut body);
    append_field_header(3, 0, &mut header);
    append_varint(body.len() as u64, &mut header);

    file.extend_from_slice(&u32::try_from(header.len()).unwrap().to_be_bytes());
    file.extend_from_slice(&header);
    file.extend_from_slice(&body);
}

fn write_fixture(
    plain_nodes: &[FixtureNode],
    dense_nodes: &[FixtureNode],
    ways: &[FixtureWay],
    relations: &[FixtureRelation],
) -> (tempfile::TempDir, PathBuf) {
    let mut header_block = Vec::new();
    append_bytes_field(4, b"OsmSchema-V0.6", &mut header_block);
    append_bytes_field(4, b"DenseNodes", &mut header_block);

    let mut strings = vec![String::new()];
    let mut groups = Vec::new();
    append_plain_node_group(&mut strings, plain_nodes, &mut groups);
    append_dense_node_group(&mut strings, dense_nodes, &mut groups);
    append_way_group(&mut strings, ways, &mut groups);
    append_relation_group(&mut strings, relations, &mut groups);

    let mut string_table = Vec::new();
    for string in &strings {
        append_bytes_field(1, string.as_bytes(), &mut string_table);
    }
    let mut block = Vec::new();
    append_bytes_field(1, &string_table, &mut block);
    block.extend_from_slice(&groups);

    let mut file = Vec::new();
    append_blob("OSMHeader", &header_block, &mut file);
    append_blob("OSMData", &block, &mut file);

    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("fixture.osm.pbf");
    std::fs::write(&path, &file).unwrap();
    (directory, path)
}

fn node(id: i64, latitude: f64, longitude: f64) -> FixtureNode {
    FixtureNode {
        id,
        latitude,
        longitude,
        tags: Vec::new(),
    }
}

fn sample_fixture() -> (tempfile::TempDir, PathBuf) {
    write_fixture(
        &[FixtureNode {
            id: 6,
            latitude: 34.6,
            longitude: 134.6,
            tags: vec![("highway", "motorway_junction"), ("name", "分岐")],
        }],
        &[
            node(1, 35.1, 139.1),
            node(2, 35.2, 139.2),
            node(3, 35.3, 139.3),
            node(4, 35.4, 139.4),
            FixtureNode {
                id: 5,
                latitude: 34.5,
                longitude: 134.5,
                tags: vec![("railway", "station"), ("name", "駅")],
            },
            node(7, 33.0, 133.0),
        ],
        &[
            FixtureWay {
                id: 100,
                tags: vec![("highway", "motorway")],
                node_ids: vec![1, 2, 3],
            },
            FixtureWay {
                id: 101,
                tags: vec![("highway", "residential")],
                node_ids: vec![3, 4],
            },
            FixtureWay {
                id: 102,
                tags: vec![("highway", "motorway")],
                node_ids: vec![4, 999],
            },
        ],
        &[
            FixtureRelation {
                id: 300,
                tags: vec![("type", "route"), ("route", "road"), ("ref", "1")],
                way_members: vec![("", 101), ("stop", 100)],
            },
            FixtureRelation {
                id: 301,
                tags: vec![("type", "multipolygon")],
                way_members: vec![("", 100)],
            },
        ],
    )
}

fn sample_filter() -> ExtractionFilter {
    ExtractionFilter {
        point: Box::new(|tags: &Tags| tags.value_of("railway") == Some("station")),
        line: Box::new(|tags: &Tags| tags.value_of("highway") == Some("motorway")),
        relation: Box::new(|tags: &Tags| tags.value_of("route") == Some("road")),
        member_role: Box::new(str::is_empty),
        include_relation_member_lines: true,
    }
}

#[test]
fn extracts_points_from_tagged_dense_nodes() {
    let (_directory, path) = sample_fixture();

    let extraction = extract(&path, &sample_filter()).unwrap();

    assert_eq!(extraction.points.len(), 1);
    let point = &extraction.points[0];
    assert_eq!(point.id, 5);
    assert_eq!(point.tags.value_of("name"), Some("駅"));
    assert!((point.latitude - 34.5).abs() < 1e-6);
    assert!((point.longitude - 134.5).abs() < 1e-6);
}

#[test]
fn extracts_points_from_tagged_plain_nodes() {
    let (_directory, path) = sample_fixture();
    let mut filter = sample_filter();
    filter.point = Box::new(|tags: &Tags| tags.value_of("highway") == Some("motorway_junction"));

    let extraction = extract(&path, &filter).unwrap();

    assert_eq!(extraction.points.len(), 1);
    assert_eq!(extraction.points[0].id, 6);
    assert_eq!(extraction.points[0].tags.value_of("name"), Some("分岐"));
}

#[test]
fn does_not_extract_untagged_nodes_as_points() {
    let (_directory, path) = sample_fixture();
    let mut filter = sample_filter();
    filter.point = Box::new(|_| true);

    let extraction = extract(&path, &filter).unwrap();

    assert!(!extraction.points.iter().any(|point| point.id == 7));
}

#[test]
fn extracts_lines_with_coordinates_resolved_in_way_order() {
    let (_directory, path) = sample_fixture();

    let extraction = extract(&path, &sample_filter()).unwrap();

    let line = extraction.lines.iter().find(|line| line.id == 100).unwrap();
    assert_eq!(line.tags.value_of("highway"), Some("motorway"));
    assert_eq!(line.coordinates.len(), 3);
    for (coordinate, expected) in
        line.coordinates
            .iter()
            .zip([(35.1, 139.1), (35.2, 139.2), (35.3, 139.3)])
    {
        assert!((coordinate.0 - expected.0).abs() < 1e-6);
        assert!((coordinate.1 - expected.1).abs() < 1e-6);
    }
}

#[test]
fn drops_lines_with_fewer_than_two_resolved_coordinates() {
    let (_directory, path) = sample_fixture();

    let extraction = extract(&path, &sample_filter()).unwrap();

    assert!(!extraction.lines.iter().any(|line| line.id == 102));
}

#[test]
fn includes_relation_member_ways_as_lines() {
    let (_directory, path) = sample_fixture();

    let extraction = extract(&path, &sample_filter()).unwrap();

    let member_line = extraction.lines.iter().find(|line| line.id == 101).unwrap();
    assert_eq!(member_line.tags.value_of("highway"), Some("residential"));
    assert_eq!(member_line.coordinates.len(), 2);
}

#[test]
fn excludes_relation_member_ways_when_disabled() {
    let (_directory, path) = sample_fixture();
    let mut filter = sample_filter();
    filter.include_relation_member_lines = false;

    let extraction = extract(&path, &filter).unwrap();

    assert_eq!(
        extraction
            .lines
            .iter()
            .map(|line| line.id)
            .collect::<Vec<_>>(),
        vec![100]
    );
}

#[test]
fn extracts_relations_with_members_filtered_by_role() {
    let (_directory, path) = sample_fixture();

    let extraction = extract(&path, &sample_filter()).unwrap();

    assert_eq!(extraction.relations.len(), 1);
    let relation = &extraction.relations[0];
    assert_eq!(relation.id, 300);
    assert_eq!(relation.tags.value_of("ref"), Some("1"));
    assert_eq!(relation.member_way_ids, vec![101]);
}

#[test]
fn returns_features_sorted_by_id() {
    let (_directory, path) = sample_fixture();
    let mut filter = sample_filter();
    filter.point = Box::new(|_| true);

    let extraction = extract(&path, &filter).unwrap();

    let line_ids: Vec<i64> = extraction.lines.iter().map(|line| line.id).collect();
    let point_ids: Vec<i64> = extraction.points.iter().map(|point| point.id).collect();
    assert_eq!(line_ids, vec![100, 101]);
    assert_eq!(point_ids, vec![5, 6]);
}
