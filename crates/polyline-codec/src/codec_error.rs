use std::error::Error;
use std::fmt;

/// 符号化されたバイト列の復号の失敗を表すエラー。
#[derive(Debug, PartialEq, Eq)]
pub enum CodecError {
    /// バイト列が座標の途中で終端している。
    UnexpectedEnd,
}

impl fmt::Display for CodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEnd => write!(f, "バイト列が座標の途中で終端している"),
        }
    }
}

impl Error for CodecError {}
