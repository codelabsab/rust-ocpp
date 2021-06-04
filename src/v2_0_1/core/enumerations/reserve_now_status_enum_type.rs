#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum ReserveNowStatusEnumType {
    Accepted,
    Faulted,
    Occupied,
    Rejected,
    Unavailable,
}
