mod write_signed_varint;

use write_signed_varint::write_signed_varint;

/// 座標列を、成分ごとに `scale` 倍して最近接整数へ量子化し、直前の点との差分（先頭の点は 0 との差分)を点ごとに第 1 成分・第 2 成分の順で zigzag varint として並べたバイト列へ符号化する。差分は 2 の 64 乗を法とする折り返し演算で計算する。事前条件として、量子化後の各成分の値は `i64` の範囲に収まること。
#[must_use]
pub fn encode_coordinates(coordinates: &[(f64, f64)], scale: f64) -> Vec<u8> {
    let mut bytes = Vec::new();

    let mut previous = (0_i64, 0_i64);
    for &(first, second) in coordinates {
        #[allow(
            clippy::cast_possible_truncation,
            reason = "量子化後の値が i64 の範囲に収まることを事前条件とする"
        )]
        let quantized = (
            (first * scale).round() as i64,
            (second * scale).round() as i64,
        );

        write_signed_varint(quantized.0.wrapping_sub(previous.0), &mut bytes);
        write_signed_varint(quantized.1.wrapping_sub(previous.1), &mut bytes);
        previous = quantized;
    }

    bytes
}
