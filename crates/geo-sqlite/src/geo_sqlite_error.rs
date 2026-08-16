use std::error::Error;
use std::fmt;

use polyline_codec::CodecError;

/// データベースの読み書きの失敗を表すエラー。
#[derive(Debug)]
pub enum GeoSqliteError {
    /// `SQLite` の操作に失敗した。
    Sqlite(rusqlite::Error),
    /// 格納された座標列の復号に失敗した。
    Codec(CodecError),
    /// ファイルの操作に失敗した。
    Io(std::io::Error),
    /// 折れ線の種別が指定されておらず、種別を持つグループにも属していない。
    UnresolvedLineKind(i64),
    /// 折れ線が座標を持っていない。
    EmptyLine(i64),
}

impl fmt::Display for GeoSqliteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(source) => write!(f, "SQLite の操作に失敗した: {source}"),
            Self::Codec(source) => write!(f, "格納された座標列の復号に失敗した: {source}"),
            Self::Io(source) => write!(f, "ファイルの操作に失敗した: {source}"),
            Self::UnresolvedLineKind(id) => {
                write!(
                    f,
                    "折れ線 {id} の種別が指定されておらず、グループからも継承できない"
                )
            }
            Self::EmptyLine(id) => write!(f, "折れ線 {id} が座標を持っていない"),
        }
    }
}

impl Error for GeoSqliteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Sqlite(source) => Some(source),
            Self::Codec(source) => Some(source),
            Self::Io(source) => Some(source),
            Self::UnresolvedLineKind(_) | Self::EmptyLine(_) => None,
        }
    }
}

impl From<rusqlite::Error> for GeoSqliteError {
    fn from(source: rusqlite::Error) -> Self {
        Self::Sqlite(source)
    }
}

impl From<CodecError> for GeoSqliteError {
    fn from(source: CodecError) -> Self {
        Self::Codec(source)
    }
}

impl From<std::io::Error> for GeoSqliteError {
    fn from(source: std::io::Error) -> Self {
        Self::Io(source)
    }
}
