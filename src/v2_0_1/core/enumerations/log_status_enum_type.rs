#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum LogStatusEnumType {
    Accepted,
    Rejected,
    AcceptedCanceled,
}
