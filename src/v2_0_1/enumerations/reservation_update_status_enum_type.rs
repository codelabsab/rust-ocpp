#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ReservationUpdateStatusEnumType {
    #[default]
    Expired,
    Removed,
}
