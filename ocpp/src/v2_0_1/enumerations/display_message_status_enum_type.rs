#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum DisplayMessageStatusEnumType {
    Accepted,
    NotSupportedMessageFormat,
    Rejected,
    NotSupportedPriority,
    NotSupportedState,
    UnknownTransaction,
}
