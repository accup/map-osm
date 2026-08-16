use crate::matching::Matching;

/// キーと値の組の集合に対する 1 つの条件。キーが一致し、値が照合方法に従ってパターンに合致する組が存在することを要求する。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Condition {
    /// 照合する組のキー。
    pub key: String,
    /// 値とパターンの照合方法。
    pub matching: Matching,
    /// 値と照合するパターン。
    pub pattern: String,
}

impl Condition {
    /// キー・照合方法・パターンから条件を作る。
    #[must_use]
    pub fn new(key: impl Into<String>, matching: Matching, pattern: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            matching,
            pattern: pattern.into(),
        }
    }

    /// キーと値の組の集合がこの条件を満たすかを返す。
    #[must_use]
    pub fn is_satisfied_by(&self, tags: &[(impl AsRef<str>, impl AsRef<str>)]) -> bool {
        tags.iter().any(|(key, value)| {
            key.as_ref() == self.key && self.matching.matches(&self.pattern, value.as_ref())
        })
    }
}
