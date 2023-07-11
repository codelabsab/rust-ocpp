#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ResetStatusEnumType {
    #[default]
    Accepted,
    Rejected,
    Scheduled,
}
