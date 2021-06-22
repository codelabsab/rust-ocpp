#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum LogStatusEnumType {
    Accepted,
    Rejected,
    AcceptedCanceled,
}
