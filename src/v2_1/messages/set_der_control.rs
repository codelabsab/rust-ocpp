use crate::v2_1::datatypes::{
    CustomDataType,
    DERCurveType,
    EnterServiceType,
    FixedPFType,
    FixedVarType,
    FreqDroopType,
    GradientType,
    LimitMaxDischargeType,
    StatusInfoType,
};
use crate::v2_1::enumerations::DERControlEnumType;
use crate::v2_1::enumerations::der_control::DERControlStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the SetDERControl request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetDERControlRequest {
    /// True if this is a default DER control
    pub is_default: bool,

    /// Unique id of this control, e.g. UUID
    #[validate(length(max = 36))]
    pub control_id: String,

    pub control_type: DERControlEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub curve: Option<DERCurveType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub enter_service: Option<EnterServiceType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub fixed_pf_absorb: Option<FixedPFType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub fixed_pf_inject: Option<FixedPFType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub fixed_var: Option<FixedVarType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub freq_droop: Option<FreqDroopType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub gradient: Option<GradientType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub limit_max_discharge: Option<LimitMaxDischargeType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetDERControlRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `is_default` - True if this is a default DER control
    /// * `control_id` - Unique id of this control, e.g. UUID
    /// * `control_type` - The control_type field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(is_default: bool, control_id: String, control_type: DERControlEnumType) -> Self {
        Self {
            is_default,
            control_id,
            control_type,
            curve: None,
            enter_service: None,
            fixed_pf_absorb: None,
            fixed_pf_inject: None,
            fixed_var: None,
            freq_droop: None,
            gradient: None,
            limit_max_discharge: None,
            custom_data: None,
        }
    }

    /// Sets the is_default field.
    ///
    /// * `is_default` - True if this is a default DER control
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_is_default(&mut self, is_default: bool) -> &mut Self {
        self.is_default = is_default;
        self
    }

    /// Sets the control_id field.
    ///
    /// * `control_id` - Unique id of this control, e.g. UUID
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_control_id(&mut self, control_id: String) -> &mut Self {
        self.control_id = control_id;
        self
    }

    /// Sets the control_type field.
    ///
    /// * `control_type` - The control_type field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_control_type(&mut self, control_type: DERControlEnumType) -> &mut Self {
        self.control_type = control_type;
        self
    }

    /// Sets the curve field.
    ///
    /// * `curve` - The curve field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_curve(&mut self, curve: Option<DERCurveType>) -> &mut Self {
        self.curve = curve;
        self
    }

    /// Sets the enter_service field.
    ///
    /// * `enter_service` - The enter_service field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_enter_service(&mut self, enter_service: Option<EnterServiceType>) -> &mut Self {
        self.enter_service = enter_service;
        self
    }

    /// Sets the fixed_pf_absorb field.
    ///
    /// * `fixed_pf_absorb` - The fixed_pf_absorb field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_fixed_pf_absorb(&mut self, fixed_pf_absorb: Option<FixedPFType>) -> &mut Self {
        self.fixed_pf_absorb = fixed_pf_absorb;
        self
    }

    /// Sets the fixed_pf_inject field.
    ///
    /// * `fixed_pf_inject` - The fixed_pf_inject field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_fixed_pf_inject(&mut self, fixed_pf_inject: Option<FixedPFType>) -> &mut Self {
        self.fixed_pf_inject = fixed_pf_inject;
        self
    }

    /// Sets the fixed_var field.
    ///
    /// * `fixed_var` - The fixed_var field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_fixed_var(&mut self, fixed_var: Option<FixedVarType>) -> &mut Self {
        self.fixed_var = fixed_var;
        self
    }

    /// Sets the freq_droop field.
    ///
    /// * `freq_droop` - The freq_droop field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_freq_droop(&mut self, freq_droop: Option<FreqDroopType>) -> &mut Self {
        self.freq_droop = freq_droop;
        self
    }

    /// Sets the gradient field.
    ///
    /// * `gradient` - The gradient field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_gradient(&mut self, gradient: Option<GradientType>) -> &mut Self {
        self.gradient = gradient;
        self
    }

    /// Sets the limit_max_discharge field.
    ///
    /// * `limit_max_discharge` - The limit_max_discharge field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_limit_max_discharge(&mut self, limit_max_discharge: Option<LimitMaxDischargeType>) -> &mut Self {
        self.limit_max_discharge = limit_max_discharge;
        self
    }

    /// Sets the custom_data field.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets a reference to the is_default field.
    ///
    /// # Returns
    ///
    /// True if this is a default DER control
    pub fn get_is_default(&self) -> &bool {
        &self.is_default
    }

    /// Gets a reference to the control_id field.
    ///
    /// # Returns
    ///
    /// Unique id of this control, e.g. UUID
    pub fn get_control_id(&self) -> &String {
        &self.control_id
    }

    /// Gets a reference to the control_type field.
    ///
    /// # Returns
    ///
    /// The control_type field
    pub fn get_control_type(&self) -> &DERControlEnumType {
        &self.control_type
    }

    /// Gets a reference to the curve field.
    ///
    /// # Returns
    ///
    /// The curve field
    pub fn get_curve(&self) -> Option<&DERCurveType> {
        self.curve.as_ref()
    }

    /// Gets a reference to the enter_service field.
    ///
    /// # Returns
    ///
    /// The enter_service field
    pub fn get_enter_service(&self) -> Option<&EnterServiceType> {
        self.enter_service.as_ref()
    }

    /// Gets a reference to the fixed_pf_absorb field.
    ///
    /// # Returns
    ///
    /// The fixed_pf_absorb field
    pub fn get_fixed_pf_absorb(&self) -> Option<&FixedPFType> {
        self.fixed_pf_absorb.as_ref()
    }

    /// Gets a reference to the fixed_pf_inject field.
    ///
    /// # Returns
    ///
    /// The fixed_pf_inject field
    pub fn get_fixed_pf_inject(&self) -> Option<&FixedPFType> {
        self.fixed_pf_inject.as_ref()
    }

    /// Gets a reference to the fixed_var field.
    ///
    /// # Returns
    ///
    /// The fixed_var field
    pub fn get_fixed_var(&self) -> Option<&FixedVarType> {
        self.fixed_var.as_ref()
    }

    /// Gets a reference to the freq_droop field.
    ///
    /// # Returns
    ///
    /// The freq_droop field
    pub fn get_freq_droop(&self) -> Option<&FreqDroopType> {
        self.freq_droop.as_ref()
    }

    /// Gets a reference to the gradient field.
    ///
    /// # Returns
    ///
    /// The gradient field
    pub fn get_gradient(&self) -> Option<&GradientType> {
        self.gradient.as_ref()
    }

    /// Gets a reference to the limit_max_discharge field.
    ///
    /// # Returns
    ///
    /// The limit_max_discharge field
    pub fn get_limit_max_discharge(&self) -> Option<&LimitMaxDischargeType> {
        self.limit_max_discharge.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the curve field and returns self for builder pattern.
    ///
    /// * `curve` - The curve field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_curve(mut self, curve: DERCurveType) -> Self {
        self.curve = Some(curve);
        self
    }

    /// Sets the enter_service field and returns self for builder pattern.
    ///
    /// * `enter_service` - The enter_service field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_enter_service(mut self, enter_service: EnterServiceType) -> Self {
        self.enter_service = Some(enter_service);
        self
    }

    /// Sets the fixed_pf_absorb field and returns self for builder pattern.
    ///
    /// * `fixed_pf_absorb` - The fixed_pf_absorb field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_fixed_pf_absorb(mut self, fixed_pf_absorb: FixedPFType) -> Self {
        self.fixed_pf_absorb = Some(fixed_pf_absorb);
        self
    }

    /// Sets the fixed_pf_inject field and returns self for builder pattern.
    ///
    /// * `fixed_pf_inject` - The fixed_pf_inject field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_fixed_pf_inject(mut self, fixed_pf_inject: FixedPFType) -> Self {
        self.fixed_pf_inject = Some(fixed_pf_inject);
        self
    }

    /// Sets the fixed_var field and returns self for builder pattern.
    ///
    /// * `fixed_var` - The fixed_var field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_fixed_var(mut self, fixed_var: FixedVarType) -> Self {
        self.fixed_var = Some(fixed_var);
        self
    }

    /// Sets the freq_droop field and returns self for builder pattern.
    ///
    /// * `freq_droop` - The freq_droop field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_freq_droop(mut self, freq_droop: FreqDroopType) -> Self {
        self.freq_droop = Some(freq_droop);
        self
    }

    /// Sets the gradient field and returns self for builder pattern.
    ///
    /// * `gradient` - The gradient field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_gradient(mut self, gradient: GradientType) -> Self {
        self.gradient = Some(gradient);
        self
    }

    /// Sets the limit_max_discharge field and returns self for builder pattern.
    ///
    /// * `limit_max_discharge` - The limit_max_discharge field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_limit_max_discharge(mut self, limit_max_discharge: LimitMaxDischargeType) -> Self {
        self.limit_max_discharge = Some(limit_max_discharge);
        self
    }

    /// Sets the custom_data field and returns self for builder pattern.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

}

/// Response body for the SetDERControl response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetDERControlResponse {
    pub status: DERControlStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// List of controlIds that are superseded as a result of setting this control.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    pub superseded_ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetDERControlResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: DERControlStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
            superseded_ids: None,
            custom_data: None,
        }
    }

    /// Sets the status field.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status(&mut self, status: DERControlStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Sets the status_info field.
    ///
    /// * `status_info` - The status_info field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status_info(&mut self, status_info: Option<StatusInfoType>) -> &mut Self {
        self.status_info = status_info;
        self
    }

    /// Sets the superseded_ids field.
    ///
    /// * `superseded_ids` - List of controlIds that are superseded as a result of setting this control.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_superseded_ids(&mut self, superseded_ids: Option<Vec<String>>) -> &mut Self {
        self.superseded_ids = superseded_ids;
        self
    }

    /// Sets the custom_data field.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets a reference to the status field.
    ///
    /// # Returns
    ///
    /// The status field
    pub fn get_status(&self) -> &DERControlStatusEnumType {
        &self.status
    }

    /// Gets a reference to the status_info field.
    ///
    /// # Returns
    ///
    /// The status_info field
    pub fn get_status_info(&self) -> Option<&StatusInfoType> {
        self.status_info.as_ref()
    }

    /// Gets a reference to the superseded_ids field.
    ///
    /// # Returns
    ///
    /// List of controlIds that are superseded as a result of setting this control.
    pub fn get_superseded_ids(&self) -> Option<&Vec<String>> {
        self.superseded_ids.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the status_info field and returns self for builder pattern.
    ///
    /// * `status_info` - The status_info field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_status_info(mut self, status_info: StatusInfoType) -> Self {
        self.status_info = Some(status_info);
        self
    }

    /// Sets the superseded_ids field and returns self for builder pattern.
    ///
    /// * `superseded_ids` - List of controlIds that are superseded as a result of setting this control.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_superseded_ids(mut self, superseded_ids: Vec<String>) -> Self {
        self.superseded_ids = Some(superseded_ids);
        self
    }

    /// Sets the custom_data field and returns self for builder pattern.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
    use crate::v2_1::enumerations::{DERControlEnumType, der_control::DERControlStatusEnumType};

    #[test]
    fn test_set_der_control_request_new() {
        let request = SetDERControlRequest::new(true, "test-control-id".to_string(), DERControlEnumType::EnterService);

        assert_eq!(request.is_default, true);
        assert_eq!(request.control_id, "test-control-id");
        assert_eq!(request.control_type, DERControlEnumType::EnterService);
        assert_eq!(request.curve, None);
        assert_eq!(request.enter_service, None);
        assert_eq!(request.fixed_pf_absorb, None);
        assert_eq!(request.fixed_pf_inject, None);
        assert_eq!(request.fixed_var, None);
        assert_eq!(request.freq_droop, None);
        assert_eq!(request.gradient, None);
        assert_eq!(request.limit_max_discharge, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_set_der_control_request_serialization() {
        let request = SetDERControlRequest::new(false, "uuid-123".to_string(), DERControlEnumType::FreqDroop);

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: SetDERControlRequest = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert!(json.contains("\"isDefault\":false"));
        assert!(json.contains("\"controlId\":\"uuid-123\""));
        assert!(json.contains("\"controlType\":\"FreqDroop\""));
    }

    #[test]
    fn test_set_der_control_request_validation() {
        // Test valid request
        let valid_request = SetDERControlRequest::new(true, "valid-id".to_string(), DERControlEnumType::EnterService);
        assert!(valid_request.validate().is_ok());

        // Test invalid request with control_id too long
        let invalid_request = SetDERControlRequest::new(true, "a".repeat(37), DERControlEnumType::EnterService);
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_set_der_control_request_builder_pattern() {
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let request = SetDERControlRequest::new(true, "builder-test".to_string(), DERControlEnumType::EnterService)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.is_default, true);
        assert_eq!(request.control_id, "builder-test");
        assert_eq!(request.control_type, DERControlEnumType::EnterService);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_der_control_request_setters() {
        let mut request = SetDERControlRequest::new(false, "initial".to_string(), DERControlEnumType::EnterService);
        let custom_data = CustomDataType::new("test_vendor".to_string());

        request.set_is_default(true)
               .set_control_id("updated".to_string())
               .set_control_type(DERControlEnumType::FreqDroop)
               .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.is_default, true);
        assert_eq!(request.control_id, "updated");
        assert_eq!(request.control_type, DERControlEnumType::FreqDroop);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_der_control_request_getters() {
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let request = SetDERControlRequest::new(true, "getter-test".to_string(), DERControlEnumType::EnterService)
            .with_custom_data(custom_data.clone());

        assert_eq!(*request.get_is_default(), true);
        assert_eq!(request.get_control_id(), &"getter-test".to_string());
        assert_eq!(*request.get_control_type(), DERControlEnumType::EnterService);
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_der_control_response_new() {
        let response = SetDERControlResponse::new(DERControlStatusEnumType::Accepted);
        assert_eq!(response.status, DERControlStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.superseded_ids, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_set_der_control_response_serialization() {
        let response = SetDERControlResponse::new(DERControlStatusEnumType::Rejected);

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: SetDERControlResponse = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(json.contains("\"status\":\"rejected\""));
    }

    #[test]
    fn test_set_der_control_response_builder_pattern() {
        let status_info = StatusInfoType::new("Control conflict".to_string());
        let superseded_ids = vec!["old-id-1".to_string(), "old-id-2".to_string()];
        let custom_data = CustomDataType::new("test_vendor".to_string());
        let response = SetDERControlResponse::new(DERControlStatusEnumType::NotSupported)
            .with_status_info(status_info.clone())
            .with_superseded_ids(superseded_ids.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, DERControlStatusEnumType::NotSupported);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.superseded_ids, Some(superseded_ids));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_der_control_response_setters() {
        let mut response = SetDERControlResponse::new(DERControlStatusEnumType::Accepted);
        let status_info = StatusInfoType::new("Updated status".to_string());
        let superseded_ids = vec!["superseded-1".to_string()];
        let custom_data = CustomDataType::new("TestVendor".to_string());

        response.set_status(DERControlStatusEnumType::Rejected)
                .set_status_info(Some(status_info.clone()))
                .set_superseded_ids(Some(superseded_ids.clone()))
                .set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, DERControlStatusEnumType::Rejected);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.superseded_ids, Some(superseded_ids));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_set_der_control_response_getters() {
        let status_info = StatusInfoType::new("Test status".to_string());
        let superseded_ids = vec!["test-id".to_string()];
        let custom_data = CustomDataType::new("TestVendor".to_string());
        let response = SetDERControlResponse::new(DERControlStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_superseded_ids(superseded_ids.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(*response.get_status(), DERControlStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_superseded_ids(), Some(&superseded_ids));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_set_der_control_edge_cases() {
        // Test with maximum allowed control_id length
        let max_control_id = "a".repeat(36);
        let request = SetDERControlRequest::new(true, max_control_id.clone(), DERControlEnumType::EnterService);
        assert!(request.validate().is_ok());
        assert_eq!(request.control_id, max_control_id);

        // Test response with maximum superseded_ids
        let max_superseded_ids: Vec<String> = (0..24).map(|i| format!("id-{}", i)).collect();
        let mut response = SetDERControlResponse::new(DERControlStatusEnumType::Accepted);
        response.superseded_ids = Some(max_superseded_ids.clone());
        assert!(response.validate().is_ok());

        // Test response with too many superseded_ids (should fail)
        let too_many_ids: Vec<String> = (0..25).map(|i| format!("id-{}", i)).collect();
        response.superseded_ids = Some(too_many_ids);
        assert!(response.validate().is_err());

        // Test response with empty superseded_ids (should fail)
        response.superseded_ids = Some(vec![]);
        assert!(response.validate().is_err());
    }

    #[test]
    fn test_set_der_control_response_validation() {
        let response = SetDERControlResponse::new(DERControlStatusEnumType::Accepted);
        assert!(response.validate().is_ok());
    }
}