use chrono::{DateTime, Utc};

use crate::v2_0_1::core::enumerations::connector_status_enum_type::ConnectorStatusEnumType;

/// Sent by the Charging Station to the CSMS to request that the Certificate Authority signs the public key into a certificate.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationRequest {
    pub timestamp: DateTime<Utc>,
    pub connector_status: ConnectorStatusEnumType,
    pub evse_id: i64,
    pub connector_id: i64,
}

/// This contains the field definition of StatusNotificationResponse sent by the CSMS to the Charging Station in response to a StatusNotificationRequest. This message is deprecated. This message might be removed in a future version of OCPP. It will be replaced by Device Management Monitoring events.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationResponse {}
