use std::path::Path;

use geo_sqlite::{
    COORDINATE_SCALE, DatabaseContent, GeoSqliteError, Line, LineGroup, Point, read_database,
    write_database,
};

fn sample_content() -> DatabaseContent {
    DatabaseContent {
        metadata: vec![("attribution".to_string(), "テスト".to_string())],
        line_groups: vec![LineGroup {
            kind: 3,
            reference: Some("1".to_string()),
            name: Some("路線".to_string()),
            member_line_ids: vec![100, 101],
        }],
        lines: vec![
            Line {
                id: 100,
                kind: None,
                coordinates: vec![(35.0, 139.0), (35.001, 139.001)],
            },
            Line {
                id: 101,
                kind: Some(1),
                coordinates: vec![(36.0, 140.0), (36.001, 140.001)],
            },
        ],
        points: vec![Point {
            id: 200,
            kind: 5,
            reference: Some("JY17".to_string()),
            name: Some("駅".to_string()),
            latitude: 35.5,
            longitude: 139.5,
        }],
    }
}

fn write_sample(path: &Path) -> DatabaseContent {
    let content = sample_content();
    write_database(path, &content).unwrap();
    content
}

#[test]
fn roundtrips_database_content() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("features.sqlite");

    let written = write_sample(&path);
    let read = read_database(&path).unwrap();

    assert_eq!(read.metadata, written.metadata);
    assert_eq!(read.line_groups, written.line_groups);
    assert_eq!(read.points, written.points);
    assert_eq!(read.lines.len(), written.lines.len());
    for (read_line, written_line) in read.lines.iter().zip(&written.lines) {
        assert_eq!(read_line.id, written_line.id);
        for (read_point, written_point) in
            read_line.coordinates.iter().zip(&written_line.coordinates)
        {
            assert!((read_point.0 - written_point.0).abs() < 1e-6);
            assert!((read_point.1 - written_point.1).abs() < 1e-6);
        }
    }
}

#[test]
fn resolves_line_kind_from_the_first_containing_group() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("features.sqlite");

    write_sample(&path);
    let read = read_database(&path).unwrap();

    assert_eq!(read.lines[0].kind, Some(3));
    assert_eq!(read.lines[1].kind, Some(1));
}

#[test]
fn fails_for_line_without_kind_and_group() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("features.sqlite");
    let mut content = sample_content();
    content.line_groups.clear();

    let error = write_database(&path, &content).unwrap_err();

    assert!(matches!(error, GeoSqliteError::UnresolvedLineKind(100)));
}

#[test]
fn fails_for_line_without_coordinates() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("features.sqlite");
    let mut content = sample_content();
    content.lines[1].coordinates.clear();

    let error = write_database(&path, &content).unwrap_err();

    assert!(matches!(error, GeoSqliteError::EmptyLine(101)));
}

#[test]
fn ignores_member_ids_missing_from_lines() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("features.sqlite");
    let mut content = sample_content();
    content.line_groups[0].member_line_ids = vec![100, 101, 999];

    write_database(&path, &content).unwrap();
    let read = read_database(&path).unwrap();

    assert_eq!(read.line_groups[0].member_line_ids, vec![100, 101]);
}

#[test]
fn replaces_an_existing_database_file() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("features.sqlite");
    let mut content = write_sample(&path);

    content.points.clear();
    write_database(&path, &content).unwrap();
    let read = read_database(&path).unwrap();

    assert_eq!(read.points, Vec::new());
}

#[test]
fn preserves_the_existing_database_when_writing_fails() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("features.sqlite");
    let written = write_sample(&path);

    let mut invalid = sample_content();
    invalid.line_groups.clear();
    write_database(&path, &invalid).unwrap_err();
    let read = read_database(&path).unwrap();

    assert_eq!(read.line_groups, written.line_groups);
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 1);
}

#[test]
fn stores_line_bounds_in_the_spatial_index() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("features.sqlite");

    write_sample(&path);

    let connection = rusqlite::Connection::open(&path).unwrap();
    let found: Vec<i64> = connection
        .prepare(
            "SELECT id FROM line_index
            WHERE max_latitude >= 34.9 AND min_latitude <= 35.1
                AND max_longitude >= 138.9 AND min_longitude <= 139.1
            ORDER BY id",
        )
        .unwrap()
        .query_map((), |row| row.get(0))
        .unwrap()
        .collect::<rusqlite::Result<Vec<_>>>()
        .unwrap();

    assert_eq!(found, vec![100]);
}

#[test]
fn stores_coordinates_as_documented_delta_varint_blobs() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("features.sqlite");

    let written = write_sample(&path);

    let connection = rusqlite::Connection::open(&path).unwrap();
    let blob: Vec<u8> = connection
        .query_row("SELECT coordinates FROM line WHERE id = 100", (), |row| {
            row.get(0)
        })
        .unwrap();
    let decoded = polyline_codec::decode_coordinates(&blob, COORDINATE_SCALE).unwrap();

    assert_eq!(decoded.len(), written.lines[0].coordinates.len());
    assert!((decoded[0].0 - 35.0).abs() < 1e-6);
    assert!((decoded[0].1 - 139.0).abs() < 1e-6);
}
