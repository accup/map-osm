/// パターンと値の照合方法。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Matching {
    /// 値がパターンと完全に一致する。
    Exact,
    /// 値がパターンで始まる。
    Prefix,
}

impl Matching {
    /// 照合方法に従って値がパターンに合致するかを返す。
    #[must_use]
    pub fn matches(self, pattern: &str, value: &str) -> bool {
        match self {
            Self::Exact => value == pattern,
            Self::Prefix => value.starts_with(pattern),
        }
    }
}
