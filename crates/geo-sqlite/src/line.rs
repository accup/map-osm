/// 折れ線の地物。座標は度単位の（緯度, 経度）で表現する。
#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    /// 折れ線の識別子。
    pub id: i64,
    /// 折れ線の種別。`None` の場合は所属するグループの種別を継承する。
    pub kind: Option<i64>,
    /// 折れ線を構成する座標列。
    pub coordinates: Vec<(f64, f64)>,
}
