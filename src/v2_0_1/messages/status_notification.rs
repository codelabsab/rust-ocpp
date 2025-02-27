use chrono::DateTime;
use chrono::Utc;

use crate::v2_0_1::enumerations::connector_status_enum_type::ConnectorStatusEnumType;
use crate::v2_0_1::helpers::datetime_rfc3339;

/// Sent by the Charging Station to the CSMS to request that the Certificate Authority signs the public key into a certificate.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationRequest {
    #[serde(with = "datetime_rfc3339 ")]
    pub timestamp: DateTime<Utc>,
    pub connector_status: ConnectorStatusEnumType,
    pub evse_id: i32,
    pub connector_id: i32,
}

/// This contains the field definition of StatusNotificationResponse sent by the CSMS to the Charging Station in response to a StatusNotificationRequest. This message is deprecated. This message might be removed in a future version of OCPP. It will be replaced by Device Management Monitoring events.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationResponse {}
