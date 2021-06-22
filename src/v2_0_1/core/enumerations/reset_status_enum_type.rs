#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ResetStatusEnumType {
    Accepted,
    Rejected,
    Scheduled,
}
