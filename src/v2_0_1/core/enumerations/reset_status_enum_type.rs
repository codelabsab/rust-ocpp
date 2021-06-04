#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum ResetStatusEnumType {
    Accepted,
    Rejected,
    Scheduled,
}
