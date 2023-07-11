#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum DataEnumType {
    #[default]
    #[serde(rename = "string")]
    String,
    #[serde(rename = "decimal")]
    Decimal,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "dateTime")]
    Datetime,
    #[serde(rename = "boolean")]
    Boolean,
    OptionList,
    SequenceList,
    MemberList,
}
