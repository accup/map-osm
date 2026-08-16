/// 点の地物。座標は度単位で表現する。
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    /// 点の識別子。
    pub id: i64,
    /// 点の種別。
    pub kind: i64,
    /// 点の付番。
    pub reference: Option<String>,
    /// 点の名称。
    pub name: Option<String>,
    /// 度単位の緯度。
    pub latitude: f64,
    /// 度単位の経度。
    pub longitude: f64,
}
