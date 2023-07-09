#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum CancelReservationStatusEnumType {
    #[default]
    Accepted,
    Rejected,
}
