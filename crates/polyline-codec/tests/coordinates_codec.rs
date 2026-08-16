use polyline_codec::{CodecError, decode_coordinates, encode_coordinates};

#[test]
fn encodes_empty_coordinates_to_empty_bytes() {
    assert_eq!(encode_coordinates(&[], 1e6), Vec::<u8>::new());
}

#[test]
fn encodes_deltas_as_zigzag_varints() {
    let coordinates = [(0.0, 0.0), (0.000_001, -0.000_001), (0.000_001, -0.000_001)];

    let bytes = encode_coordinates(&coordinates, 1e6);

    assert_eq!(bytes, vec![0, 0, 2, 1, 0, 0]);
}

#[test]
fn encodes_first_coordinate_as_delta_from_zero() {
    let bytes = encode_coordinates(&[(0.000_064, -0.000_064)], 1e6);

    assert_eq!(bytes, vec![0x80, 0x01, 0x7f]);
}

#[test]
fn roundtrips_quantized_coordinates() {
    let coordinates = [
        (35.681_236, 139.767_125),
        (35.681_240, 139.767_100),
        (34.702_485, 135.495_951),
        (-35.5, -139.5),
    ];

    let decoded = decode_coordinates(&encode_coordinates(&coordinates, 1e6), 1e6).unwrap();

    assert_eq!(decoded.len(), coordinates.len());
    for ((decoded_first, decoded_second), (first, second)) in decoded.iter().zip(&coordinates) {
        assert!((decoded_first - first).abs() < 1e-6);
        assert!((decoded_second - second).abs() < 1e-6);
    }
}

#[test]
fn quantizes_coordinates_to_scale_resolution() {
    let decoded =
        decode_coordinates(&encode_coordinates(&[(0.000_001_4, 0.000_001_6)], 1e6), 1e6).unwrap();

    assert_eq!(decoded, vec![(0.000_001, 0.000_002)]);
}

#[test]
fn decodes_empty_bytes_to_empty_coordinates() {
    assert_eq!(decode_coordinates(&[], 1e6).unwrap(), Vec::new());
}

#[test]
fn fails_to_decode_bytes_ending_in_the_middle_of_a_coordinate() {
    assert_eq!(
        decode_coordinates(&[0], 1e6),
        Err(CodecError::UnexpectedEnd)
    );
}

#[test]
fn fails_to_decode_bytes_ending_in_the_middle_of_a_varint() {
    assert_eq!(
        decode_coordinates(&[0x80], 1e6),
        Err(CodecError::UnexpectedEnd)
    );
}

#[test]
fn fails_to_decode_a_varint_longer_than_64_bits() {
    let bytes = [
        0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01,
    ];

    assert_eq!(
        decode_coordinates(&bytes, 1e6),
        Err(CodecError::VarintTooLong)
    );
}
