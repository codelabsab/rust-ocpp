/// Defines parameters required for initiating and maintaining wireless communication with other devices.
/// ModemType is used by: BootNotificationRequest.ChargingStationType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModemType<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iccid: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<&'a str>,
}
