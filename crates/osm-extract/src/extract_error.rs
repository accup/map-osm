use std::error::Error;
use std::fmt;

/// OSM PBF ファイルからの抽出の失敗を表すエラー。
#[derive(Debug)]
pub struct ExtractError {
    source: osmpbf::Error,
}

impl ExtractError {
    pub(crate) fn new(source: osmpbf::Error) -> Self {
        Self { source }
    }
}

impl fmt::Display for ExtractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OSM PBF ファイルからの抽出に失敗した: {}", self.source)
    }
}

impl Error for ExtractError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
