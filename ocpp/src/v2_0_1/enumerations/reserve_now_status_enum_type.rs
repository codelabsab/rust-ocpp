#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ReserveNowStatusEnumType {
    Accepted,
    Faulted,
    Occupied,
    Rejected,
    Unavailable,
}
