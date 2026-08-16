use crate::rule::Rule;

/// キーと値の組の集合を規則の表で分類し、すべての条件を満たす最初の規則のラベルを返す。満たす規則がない場合は `None` を返す。
#[must_use]
pub fn classify<T: Clone>(
    rules: &[Rule<T>],
    tags: &[(impl AsRef<str>, impl AsRef<str>)],
) -> Option<T> {
    rules
        .iter()
        .find(|rule| rule.is_satisfied_by(tags))
        .map(|rule| rule.label.clone())
}
