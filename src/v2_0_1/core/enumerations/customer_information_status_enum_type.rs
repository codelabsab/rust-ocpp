#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum CustomerInformationStatusEnumType {
    Accepted,
    Rejected,
    Invalid,
}
