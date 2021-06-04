#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum DataEnumType {
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
