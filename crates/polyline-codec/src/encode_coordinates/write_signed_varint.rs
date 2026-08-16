/// 符号付き整数を zigzag 符号化し、下位 7 ビットずつ継続ビット付きの varint としてバイト列の末尾へ書き込む。
#[allow(
    clippy::cast_sign_loss,
    reason = "zigzag 符号化は符号ビットを最下位ビットへ移すため、ビット表現の再解釈が意図した動作である"
)]
pub(crate) fn write_signed_varint(value: i64, bytes: &mut Vec<u8>) {
    let mut zigzag = ((value << 1) ^ (value >> 63)) as u64;

    while zigzag >= 0x80 {
        #[allow(
            clippy::cast_possible_truncation,
            reason = "下位 7 ビットのみを取り出す"
        )]
        bytes.push((zigzag & 0x7f) as u8 | 0x80);
        zigzag >>= 7;
    }
    #[allow(
        clippy::cast_possible_truncation,
        reason = "剰余のビット数は 7 以下である"
    )]
    bytes.push(zigzag as u8);
}
