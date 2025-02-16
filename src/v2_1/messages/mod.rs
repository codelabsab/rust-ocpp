pub mod adjust_periodic_event_stream;
pub mod afrr_signal;
pub mod authorize;
pub mod battery_swap;
pub mod boot_notification;
pub mod cancel_reservation;
pub mod certificate_signed;
pub mod status_notification;
pub mod transaction_event;
pub mod unlock_connector;
pub mod unpublish_firmware;
pub mod update_firmware;
pub mod use_priority_charging;
pub mod vat_number_validation;

// Re-exports
pub use crate::v2_1::datatypes::custom_data::CustomDataType as CustomData;
pub use crate::v2_1::datatypes::{
    address::AddressType as Address, evse::EVSEType as EVSE, firmware::FirmwareType as Firmware,
    id_token::IdTokenType as IdToken, id_token_info::IdTokenInfoType as IdTokenInfo,
    message_content::MessageContentType as MessageContent,
    meter_value::MeterValueType as MeterValue, status_info::StatusInfoType as StatusInfo,
    transaction::TransactionType as Transaction,
    transaction_limit::TransactionLimitType as TransactionLimit,
};
pub use crate::v2_1::enumerations::{
    generic_status::GenericStatusEnumType as GenericStatusEnum,
    priority_charging_status::PriorityChargingStatusEnumType as PriorityChargingStatusEnum,
    transaction_event::TransactionEventEnumType as TransactionEventEnum,
    trigger_reason::TriggerReasonEnumType as TriggerReasonEnum,
    unlock_status::UnlockStatusEnumType as UnlockStatusEnum,
    unpublish_firmware_status::UnpublishFirmwareStatusEnumType as UnpublishFirmwareStatusEnum,
    update_firmware_status::UpdateFirmwareStatusEnumType as UpdateFirmwareStatusEnum,
};

pub use crate::v2_1::messages::cancel_reservation::CancelReservationResponse;
pub use afrr_signal::{AFRRSignalRequest, AFRRSignalResponse};
pub use authorize::{
    AuthorizeRequest, AuthorizeResponse, HashAlgorithmEnumType, OCSPRequestDataType,
};
pub use battery_swap::{BatteryDataType, BatterySwapRequest, BatterySwapResponse};
pub use boot_notification::{BootNotificationRequest, ChargingStationType, ModemType};
pub use cancel_reservation::CancelReservationRequest;
pub use certificate_signed::{CertificateSignedRequest, CertificateSignedResponse};
pub use transaction_event::{TransactionEventRequest, TransactionEventResponse};
pub use unlock_connector::{UnlockConnectorRequest, UnlockConnectorResponse};
pub use unpublish_firmware::{UnpublishFirmwareRequest, UnpublishFirmwareResponse};
pub use update_firmware::{UpdateFirmwareRequest, UpdateFirmwareResponse};
pub use use_priority_charging::{UsePriorityChargingRequest, UsePriorityChargingResponse};
pub use vat_number_validation::{VatNumberValidationRequest, VatNumberValidationResponse};
