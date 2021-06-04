/// Contains a case insensitive identifier to use for the authorization and the type of authorization to support multiple forms of identifiers.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfoType {
    pub additional_id_token: String,
    #[serde(rename = "type")]
    pub kind: String,
}
