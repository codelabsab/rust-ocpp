use chrono::Utc;

/// tests
use crate::v2_0_1::{
    core::{
        enumerations::registration_status_enum_type::RegistrationStatusEnumType,
        messages::boot_notification::{BootNotificationRequest, BootNotificationResponse},
    },
    provisioning::ocpp_ws::OCPPWS,
};

#[async_trait::async_trait]
pub trait CSMS {
    type Error;

    async fn boot_notification_request(
        &self,
        request: BootNotificationRequest,
    ) -> Result<BootNotificationResponse, Self::Error>;
}

// define services
pub struct OCPPWSImpl<T: OCPPWS> {
    ocppws: T,
}

impl<T: OCPPWS> OCPPWSImpl<T> {
    pub fn new(ocppws: T) -> Self {
        Self { ocppws }
    }
}

#[async_trait::async_trait]
impl<T: OCPPWS> CSMS for OCPPWSImpl<T> {
    type Error = Box<dyn std::error::Error>;

    async fn boot_notification_request(
        &self,
        _request: BootNotificationRequest,
    ) -> Result<BootNotificationResponse, Self::Error> {
        self.ocppws.request().await?;
        Ok(BootNotificationResponse {
            current_time: Utc::now(),
            interval: 0,
            status: RegistrationStatusEnumType::Pending,
            status_info: None,
        })
    }
}

#[cfg(test)]

mod tests {}
