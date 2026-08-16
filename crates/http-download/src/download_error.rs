use std::error::Error;
use std::fmt;

/// HTTP ダウンロードの失敗を表すエラー。
#[derive(Debug)]
pub struct DownloadError {
    source: Box<dyn Error + Send + Sync>,
}

impl DownloadError {
    pub(crate) fn new(source: impl Into<Box<dyn Error + Send + Sync>>) -> Self {
        Self {
            source: source.into(),
        }
    }
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HTTP ダウンロードに失敗した: {}", self.source)
    }
}

impl Error for DownloadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}
