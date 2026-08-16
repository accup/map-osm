mod read_signed_varint;

use crate::codec_error::CodecError;
use read_signed_varint::read_signed_varint;

/// [`crate::encode_coordinates`] で符号化されたバイト列を、量子化された整数値を `scale` で割った座標列へ復号する。差分の累算は符号化と対称に、2 の 64 乗を法とする折り返し演算で行う。
///
/// # Errors
///
/// バイト列が座標の途中で終端している場合、または varint が 64 ビットで表現できる長さを超えている場合、エラーを返す。
pub fn decode_coordinates(bytes: &[u8], scale: f64) -> Result<Vec<(f64, f64)>, CodecError> {
    let mut coordinates = Vec::new();

    let mut rest = bytes;
    let mut previous = (0_i64, 0_i64);
    while !rest.is_empty() {
        let (first_delta, after_first) = read_signed_varint(rest)?;
        let (second_delta, after_second) = read_signed_varint(after_first)?;
        rest = after_second;

        previous = (
            previous.0.wrapping_add(first_delta),
            previous.1.wrapping_add(second_delta),
        );
        #[allow(
            clippy::cast_precision_loss,
            reason = "量子化された座標値は f64 の仮数部で表現できる大きさである"
        )]
        coordinates.push((previous.0 as f64 / scale, previous.1 as f64 / scale));
    }

    Ok(coordinates)
}
