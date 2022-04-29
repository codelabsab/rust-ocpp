#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ReservationUpdateStatusEnumType {
    Expired,
    Removed,
}
