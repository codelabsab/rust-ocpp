#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum CustomerInformationStatusEnumType {
    #[default]
    Accepted,
    Rejected,
    Invalid,
}
