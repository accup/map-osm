use crate::condition::Condition;

/// すべての条件を満たす場合にラベルを与える分類規則。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule<T> {
    /// 規則が要求する条件の集合。
    pub conditions: Vec<Condition>,
    /// 規則が与えるラベル。
    pub label: T,
}

impl<T> Rule<T> {
    /// 条件の集合とラベルから規則を作る。
    #[must_use]
    pub fn new(conditions: Vec<Condition>, label: T) -> Self {
        Self { conditions, label }
    }

    /// キーと値の組の集合がこの規則のすべての条件を満たすかを返す。
    #[must_use]
    pub fn is_satisfied_by(&self, tags: &[(impl AsRef<str>, impl AsRef<str>)]) -> bool {
        self.conditions
            .iter()
            .all(|condition| condition.is_satisfied_by(tags))
    }
}
