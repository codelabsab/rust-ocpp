#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ReserveNowStatusEnumType {
    #[default]
    Accepted,
    Faulted,
    Occupied,
    Rejected,
    Unavailable,
}
