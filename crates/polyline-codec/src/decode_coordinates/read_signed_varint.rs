use crate::codec_error::CodecError;

/// バイト列の先頭から継続ビット付きの varint を 1 つ読み取り、zigzag 復号した符号付き整数と残りのバイト列を返す。
///
/// # Errors
///
/// varint の途中でバイト列が終端している場合、または varint が 64 ビットで表現できる範囲を超えている場合、エラーを返す。
pub(crate) fn read_signed_varint(bytes: &[u8]) -> Result<(i64, &[u8]), CodecError> {
    let mut zigzag = 0_u64;
    let mut shift = 0_u32;

    for (index, &byte) in bytes.iter().enumerate() {
        if shift >= u64::BITS {
            return Err(CodecError::VarintOverflow);
        }
        if shift == u64::BITS - 1 && byte & 0x7e != 0 {
            return Err(CodecError::VarintOverflow);
        }
        zigzag |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            #[allow(
                clippy::cast_possible_wrap,
                reason = "zigzag 復号は最下位ビットを符号ビットへ戻すため、ビット表現の再解釈が意図した動作である"
            )]
            let value = (zigzag >> 1) as i64 ^ -((zigzag & 1) as i64);
            return Ok((value, &bytes[index + 1..]));
        }
        shift += 7;
    }

    Err(CodecError::UnexpectedEnd)
}
