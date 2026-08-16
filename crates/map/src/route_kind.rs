/// 路線の種別。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteKind {
    /// 鉄道。
    Railway,
    /// 高速道路。
    Expressway,
    /// 国道。
    NationalRoad,
    /// 都道府県道。
    PrefecturalRoad,
}

impl RouteKind {
    /// データベースへ格納する種別の値を返す。
    #[must_use]
    pub const fn code(self) -> i64 {
        match self {
            Self::Railway => 1,
            Self::Expressway => 2,
            Self::NationalRoad => 3,
            Self::PrefecturalRoad => 4,
        }
    }
}
