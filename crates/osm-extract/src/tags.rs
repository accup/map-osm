/// 要素に付随するキーと値の組の集合。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Tags(Vec<(String, String)>);

impl Tags {
    /// 指定されたキーを持つ最初の組の値を返す。存在しない場合は `None` を返す。
    #[must_use]
    pub fn value_of(&self, key: &str) -> Option<&str> {
        self.0
            .iter()
            .find(|(tag_key, _)| tag_key == key)
            .map(|(_, value)| value.as_str())
    }

    /// すべての組を保持された順序で返す。
    #[must_use]
    pub fn pairs(&self) -> &[(String, String)] {
        &self.0
    }
}

impl<K: Into<String>, V: Into<String>> FromIterator<(K, V)> for Tags {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(pairs: I) -> Self {
        Self(
            pairs
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}
