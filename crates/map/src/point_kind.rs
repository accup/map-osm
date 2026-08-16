/// 地点の種別。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointKind {
    /// 駅。
    Station,
    /// 高速道路のインターチェンジ・ジャンクションなどの分岐点。
    Interchange,
}

impl PointKind {
    /// データベースへ格納する種別の値を返す。
    #[must_use]
    pub const fn code(self) -> i64 {
        match self {
            Self::Station => 1,
            Self::Interchange => 2,
        }
    }
}
