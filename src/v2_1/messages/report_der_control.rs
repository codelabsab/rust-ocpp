use crate::v2_1::datatypes::{
    CustomDataType,
    DERCurveGetType,
    EnterServiceGetType,
    FixedPFGetType,
    FixedVarGetType,
    FreqDroopGetType,
    GradientGetType,
    LimitMaxDischargeGetType,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the ReportDERControl request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReportDERControlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    #[validate(nested)]
    pub curve: Option<Vec<DERCurveGetType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    #[validate(nested)]
    pub enter_service: Option<Vec<EnterServiceGetType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    #[validate(nested)]
    pub fixed_pf_absorb: Option<Vec<FixedPFGetType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    #[validate(nested)]
    pub fixed_pf_inject: Option<Vec<FixedPFGetType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    #[validate(nested)]
    pub fixed_var: Option<Vec<FixedVarGetType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    #[validate(nested)]
    pub freq_droop: Option<Vec<FreqDroopGetType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    #[validate(nested)]
    pub gradient: Option<Vec<GradientGetType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    #[validate(nested)]
    pub limit_max_discharge: Option<Vec<LimitMaxDischargeGetType>>,

    /// RequestId from GetDERControlRequest.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// To Be Continued. Default value when omitted: false. + False indicates that there are no further messages as part of this report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ReportDERControlRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - RequestId from GetDERControlRequest.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32) -> Self {
        Self {
            curve: None,
            enter_service: None,
            fixed_pf_absorb: None,
            fixed_pf_inject: None,
            fixed_var: None,
            freq_droop: None,
            gradient: None,
            limit_max_discharge: None,
            request_id,
            tbc: None,
            custom_data: None,
        }
    }

    /// Sets the curve field.
    ///
    /// * `curve` - The curve field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_curve(&mut self, curve: Option<Vec<DERCurveGetType>>) -> &mut Self {
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
    pub fn set_enter_service(&mut self, enter_service: Option<Vec<EnterServiceGetType>>) -> &mut Self {
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
    pub fn set_fixed_pf_absorb(&mut self, fixed_pf_absorb: Option<Vec<FixedPFGetType>>) -> &mut Self {
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
    pub fn set_fixed_pf_inject(&mut self, fixed_pf_inject: Option<Vec<FixedPFGetType>>) -> &mut Self {
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
    pub fn set_fixed_var(&mut self, fixed_var: Option<Vec<FixedVarGetType>>) -> &mut Self {
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
    pub fn set_freq_droop(&mut self, freq_droop: Option<Vec<FreqDroopGetType>>) -> &mut Self {
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
    pub fn set_gradient(&mut self, gradient: Option<Vec<GradientGetType>>) -> &mut Self {
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
    pub fn set_limit_max_discharge(&mut self, limit_max_discharge: Option<Vec<LimitMaxDischargeGetType>>) -> &mut Self {
        self.limit_max_discharge = limit_max_discharge;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - RequestId from GetDERControlRequest.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Sets the tbc field.
    ///
    /// * `tbc` - To Be Continued. Default value when omitted: false. + False indicates that there are no further messages as part of this report.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_tbc(&mut self, tbc: Option<bool>) -> &mut Self {
        self.tbc = tbc;
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

    /// Gets a reference to the curve field.
    ///
    /// # Returns
    ///
    /// The curve field
    pub fn get_curve(&self) -> Option<&Vec<DERCurveGetType>> {
        self.curve.as_ref()
    }

    /// Gets a reference to the enter_service field.
    ///
    /// # Returns
    ///
    /// The enter_service field
    pub fn get_enter_service(&self) -> Option<&Vec<EnterServiceGetType>> {
        self.enter_service.as_ref()
    }

    /// Gets a reference to the fixed_pf_absorb field.
    ///
    /// # Returns
    ///
    /// The fixed_pf_absorb field
    pub fn get_fixed_pf_absorb(&self) -> Option<&Vec<FixedPFGetType>> {
        self.fixed_pf_absorb.as_ref()
    }

    /// Gets a reference to the fixed_pf_inject field.
    ///
    /// # Returns
    ///
    /// The fixed_pf_inject field
    pub fn get_fixed_pf_inject(&self) -> Option<&Vec<FixedPFGetType>> {
        self.fixed_pf_inject.as_ref()
    }

    /// Gets a reference to the fixed_var field.
    ///
    /// # Returns
    ///
    /// The fixed_var field
    pub fn get_fixed_var(&self) -> Option<&Vec<FixedVarGetType>> {
        self.fixed_var.as_ref()
    }

    /// Gets a reference to the freq_droop field.
    ///
    /// # Returns
    ///
    /// The freq_droop field
    pub fn get_freq_droop(&self) -> Option<&Vec<FreqDroopGetType>> {
        self.freq_droop.as_ref()
    }

    /// Gets a reference to the gradient field.
    ///
    /// # Returns
    ///
    /// The gradient field
    pub fn get_gradient(&self) -> Option<&Vec<GradientGetType>> {
        self.gradient.as_ref()
    }

    /// Gets a reference to the limit_max_discharge field.
    ///
    /// # Returns
    ///
    /// The limit_max_discharge field
    pub fn get_limit_max_discharge(&self) -> Option<&Vec<LimitMaxDischargeGetType>> {
        self.limit_max_discharge.as_ref()
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// RequestId from GetDERControlRequest.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the tbc field.
    ///
    /// # Returns
    ///
    /// To Be Continued. Default value when omitted: false. + False indicates that there are no further messages as part of this report.
    pub fn get_tbc(&self) -> Option<&bool> {
        self.tbc.as_ref()
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
    pub fn with_curve(mut self, curve: Vec<DERCurveGetType>) -> Self {
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
    pub fn with_enter_service(mut self, enter_service: Vec<EnterServiceGetType>) -> Self {
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
    pub fn with_fixed_pf_absorb(mut self, fixed_pf_absorb: Vec<FixedPFGetType>) -> Self {
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
    pub fn with_fixed_pf_inject(mut self, fixed_pf_inject: Vec<FixedPFGetType>) -> Self {
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
    pub fn with_fixed_var(mut self, fixed_var: Vec<FixedVarGetType>) -> Self {
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
    pub fn with_freq_droop(mut self, freq_droop: Vec<FreqDroopGetType>) -> Self {
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
    pub fn with_gradient(mut self, gradient: Vec<GradientGetType>) -> Self {
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
    pub fn with_limit_max_discharge(mut self, limit_max_discharge: Vec<LimitMaxDischargeGetType>) -> Self {
        self.limit_max_discharge = Some(limit_max_discharge);
        self
    }

    /// Sets the tbc field and returns self for builder pattern.
    ///
    /// * `tbc` - To Be Continued. Default value when omitted: false. + False indicates that there are no further messages as part of this report.
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_tbc(mut self, tbc: bool) -> Self {
        self.tbc = Some(tbc);
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

/// Response body for the ReportDERControl response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReportDERControlResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ReportDERControlResponse {
    /// Creates a new instance of the struct.
    ///
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new() -> Self {
        Self {
            custom_data: None,
        }
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

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
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
    use serde_json;
    use validator::Validate;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_der_curve() -> DERCurveGetType {
        use crate::v2_1::datatypes::{DERCurveType, DERCurvePointsType};
        use crate::v2_1::enumerations::{DERControlEnumType, DERUnitEnumType};
        use rust_decimal::prelude::*;

        let curve_points = vec![DERCurvePointsType::new(
            Decimal::from_str("1.0").unwrap(),
            Decimal::from_str("2.0").unwrap(),
        )];
        let curve = DERCurveType::new(curve_points, 1, DERUnitEnumType::PctMaxW);
        DERCurveGetType::new(
            curve,
            "test_curve".to_string(),
            DERControlEnumType::FreqDroop,
            true,
            false,
        )
    }

    fn create_test_enter_service() -> EnterServiceGetType {
        use crate::v2_1::datatypes::EnterServiceType;
        use rust_decimal::prelude::*;

        let enter_service = EnterServiceType::new(
            1,
            Decimal::from_str("240.0").unwrap(),
            Decimal::from_str("220.0").unwrap(),
            Decimal::from_str("60.5").unwrap(),
            Decimal::from_str("59.5").unwrap(),
            Decimal::from_str("5.0").unwrap(),
            Decimal::from_str("2.0").unwrap(),
            Decimal::from_str("10.0").unwrap(),
        );
        EnterServiceGetType::new(enter_service, "test_enter_service".to_string())
    }

    fn create_test_fixed_pf() -> FixedPFGetType {
        use crate::v2_1::datatypes::FixedPFType;

        let fixed_pf = FixedPFType::new(1, 0.95, true);
        FixedPFGetType::new(fixed_pf, "test_fixed_pf".to_string(), false, true)
    }

    fn create_test_fixed_var() -> FixedVarGetType {
        use crate::v2_1::datatypes::FixedVarType;

        let fixed_var = FixedVarType::new(1, 100.0);
        FixedVarGetType::new(fixed_var, "test_fixed_var".to_string(), false, true)
    }

    fn create_test_freq_droop() -> FreqDroopGetType {
        use crate::v2_1::datatypes::FreqDroopType;
        use rust_decimal::prelude::*;

        let freq_droop = FreqDroopType::new(
            1,
            Decimal::from_str("60.0").unwrap(),
            Decimal::from_str("2.0").unwrap(),
            Decimal::from_str("2.0").unwrap(),
            Decimal::from_str("10.0").unwrap(),
            Decimal::from_str("5.0").unwrap(),
        );
        FreqDroopGetType::new(freq_droop, "test_freq_droop".to_string(), false, true)
    }

    fn create_test_gradient() -> GradientGetType {
        use crate::v2_1::datatypes::GradientType;
        use rust_decimal::prelude::*;

        let gradient = GradientType::new(
            1,
            Decimal::from_str("5.0").unwrap(),
            Decimal::from_str("2.0").unwrap()
        );
        GradientGetType::new(gradient, "test_gradient".to_string())
    }

    fn create_test_limit_max_discharge() -> LimitMaxDischargeGetType {
        use crate::v2_1::datatypes::LimitMaxDischargeType;
        use rust_decimal::prelude::*;

        let limit_max_discharge = LimitMaxDischargeType::new(1, Decimal::from_str("80.0").unwrap());
        LimitMaxDischargeGetType::new(limit_max_discharge, "test_limit".to_string(), false, true)
    }

    #[test]
    fn test_report_der_control_request_new() {
        let request = ReportDERControlRequest::new(123);

        assert_eq!(request.request_id, 123);
        assert!(request.curve.is_none());
        assert!(request.enter_service.is_none());
        assert!(request.fixed_pf_absorb.is_none());
        assert!(request.fixed_pf_inject.is_none());
        assert!(request.fixed_var.is_none());
        assert!(request.freq_droop.is_none());
        assert!(request.gradient.is_none());
        assert!(request.limit_max_discharge.is_none());
        assert!(request.tbc.is_none());
        assert!(request.custom_data.is_none());
    }

    #[test]
    fn test_report_der_control_request_serialization() {
        let request = ReportDERControlRequest::new(456)
            .with_curve(vec![create_test_der_curve()])
            .with_enter_service(vec![create_test_enter_service()])
            .with_tbc(true)
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&request).expect("Failed to serialize");
        let deserialized: ReportDERControlRequest =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(request, deserialized);
        assert_eq!(deserialized.request_id, 456);
        assert!(deserialized.curve.is_some());
        assert!(deserialized.enter_service.is_some());
        assert_eq!(deserialized.tbc, Some(true));
        assert!(deserialized.custom_data.is_some());
    }

    #[test]
    fn test_report_der_control_request_validation() {
        // Valid request
        let valid_request = ReportDERControlRequest::new(0);
        assert!(valid_request.validate().is_ok());

        // Invalid request_id (negative)
        let invalid_request = ReportDERControlRequest {
            curve: None,
            enter_service: None,
            fixed_pf_absorb: None,
            fixed_pf_inject: None,
            fixed_var: None,
            freq_droop: None,
            gradient: None,
            limit_max_discharge: None,
            request_id: -1,
            tbc: None,
            custom_data: None,
        };
        assert!(invalid_request.validate().is_err());

        // Test with maximum allowed vector lengths (24 items)
        let max_items = (0..24).map(|_| create_test_der_curve()).collect();
        let max_request = ReportDERControlRequest::new(1)
            .with_curve(max_items);
        assert!(max_request.validate().is_ok());

        // Test with too many items (25 items - should fail)
        let too_many_items = (0..25).map(|_| create_test_der_curve()).collect();
        let invalid_length_request = ReportDERControlRequest {
            curve: Some(too_many_items),
            enter_service: None,
            fixed_pf_absorb: None,
            fixed_pf_inject: None,
            fixed_var: None,
            freq_droop: None,
            gradient: None,
            limit_max_discharge: None,
            request_id: 1,
            tbc: None,
            custom_data: None,
        };
        assert!(invalid_length_request.validate().is_err());
    }

    #[test]
    fn test_report_der_control_request_builder_pattern() {
        let curve_data = vec![create_test_der_curve()];
        let enter_service_data = vec![create_test_enter_service()];
        let fixed_pf_data = vec![create_test_fixed_pf()];
        let custom_data = create_test_custom_data();

        let request = ReportDERControlRequest::new(789)
            .with_curve(curve_data.clone())
            .with_enter_service(enter_service_data.clone())
            .with_fixed_pf_absorb(fixed_pf_data.clone())
            .with_tbc(false)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.request_id, 789);
        assert_eq!(request.curve, Some(curve_data));
        assert_eq!(request.enter_service, Some(enter_service_data));
        assert_eq!(request.fixed_pf_absorb, Some(fixed_pf_data));
        assert_eq!(request.tbc, Some(false));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_report_der_control_request_setters_getters() {
        let mut request = ReportDERControlRequest::new(100);
        let curve_data = vec![create_test_der_curve()];
        let custom_data = create_test_custom_data();

        // Test setters
        request.set_request_id(200);
        request.set_curve(Some(curve_data.clone()));
        request.set_tbc(Some(true));
        request.set_custom_data(Some(custom_data.clone()));

        // Test getters
        assert_eq!(*request.get_request_id(), 200);
        assert_eq!(request.get_curve(), Some(&curve_data));
        assert_eq!(request.get_tbc(), Some(&true));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_report_der_control_request_all_optional_fields() {
        let request = ReportDERControlRequest::new(1)
            .with_curve(vec![create_test_der_curve()])
            .with_enter_service(vec![create_test_enter_service()])
            .with_fixed_pf_absorb(vec![create_test_fixed_pf()])
            .with_fixed_pf_inject(vec![create_test_fixed_pf()])
            .with_fixed_var(vec![create_test_fixed_var()])
            .with_freq_droop(vec![create_test_freq_droop()])
            .with_gradient(vec![create_test_gradient()])
            .with_limit_max_discharge(vec![create_test_limit_max_discharge()])
            .with_tbc(true)
            .with_custom_data(create_test_custom_data());

        assert!(request.curve.is_some());
        assert!(request.enter_service.is_some());
        assert!(request.fixed_pf_absorb.is_some());
        assert!(request.fixed_pf_inject.is_some());
        assert!(request.fixed_var.is_some());
        assert!(request.freq_droop.is_some());
        assert!(request.gradient.is_some());
        assert!(request.limit_max_discharge.is_some());
        assert_eq!(request.tbc, Some(true));
        assert!(request.custom_data.is_some());
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_report_der_control_response_new() {
        let response = ReportDERControlResponse::new();

        assert!(response.custom_data.is_none());
    }

    #[test]
    fn test_report_der_control_response_serialization() {
        let response = ReportDERControlResponse::new()
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&response).expect("Failed to serialize");
        let deserialized: ReportDERControlResponse =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(response, deserialized);
        assert!(deserialized.custom_data.is_some());
    }

    #[test]
    fn test_report_der_control_response_validation() {
        let valid_response = ReportDERControlResponse::new();
        assert!(valid_response.validate().is_ok());

        let response_with_custom_data = ReportDERControlResponse::new()
            .with_custom_data(create_test_custom_data());
        assert!(response_with_custom_data.validate().is_ok());
    }

    #[test]
    fn test_report_der_control_response_builder_pattern() {
        let custom_data = create_test_custom_data();
        let response = ReportDERControlResponse::new()
            .with_custom_data(custom_data.clone());

        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_report_der_control_response_setters_getters() {
        let mut response = ReportDERControlResponse::new();
        let custom_data = create_test_custom_data();

        // Test setter
        response.set_custom_data(Some(custom_data.clone()));

        // Test getter
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_report_der_control_request_edge_cases() {
        // Test minimum valid request_id
        let min_request = ReportDERControlRequest::new(0);
        assert!(min_request.validate().is_ok());

        // Test large request_id
        let large_request = ReportDERControlRequest::new(i32::MAX);
        assert!(large_request.validate().is_ok());

        // Test with minimum vector length (1 item)
        let min_items = vec![create_test_der_curve()];
        let min_vector_request = ReportDERControlRequest::new(1)
            .with_curve(min_items);
        assert!(min_vector_request.validate().is_ok());
    }

    #[test]
    fn test_report_der_control_request_tbc_values() {
        // Test with tbc = true
        let request_true = ReportDERControlRequest::new(1).with_tbc(true);
        assert_eq!(request_true.tbc, Some(true));

        // Test with tbc = false
        let request_false = ReportDERControlRequest::new(1).with_tbc(false);
        assert_eq!(request_false.tbc, Some(false));

        // Test without tbc (None)
        let request_none = ReportDERControlRequest::new(1);
        assert_eq!(request_none.tbc, None);
    }

    #[test]
    fn test_report_der_control_json_round_trip() {
        let original_request = ReportDERControlRequest::new(12345)
            .with_curve(vec![create_test_der_curve()])
            .with_enter_service(vec![create_test_enter_service()])
            .with_fixed_pf_absorb(vec![create_test_fixed_pf()])
            .with_tbc(true)
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&original_request).expect("Failed to serialize request");
        let parsed_request: ReportDERControlRequest =
            serde_json::from_str(&json).expect("Failed to deserialize request");

        assert_eq!(original_request, parsed_request);

        let original_response = ReportDERControlResponse::new()
            .with_custom_data(create_test_custom_data());

        let json = serde_json::to_string(&original_response).expect("Failed to serialize response");
        let parsed_response: ReportDERControlResponse =
            serde_json::from_str(&json).expect("Failed to deserialize response");

        assert_eq!(original_response, parsed_response);
    }

    #[test]
    fn test_report_der_control_request_all_getters() {
        let curve_data = vec![create_test_der_curve()];
        let enter_service_data = vec![create_test_enter_service()];
        let fixed_pf_absorb_data = vec![create_test_fixed_pf()];
        let fixed_pf_inject_data = vec![create_test_fixed_pf()];
        let fixed_var_data = vec![create_test_fixed_var()];
        let freq_droop_data = vec![create_test_freq_droop()];
        let gradient_data = vec![create_test_gradient()];
        let limit_max_discharge_data = vec![create_test_limit_max_discharge()];
        let custom_data = create_test_custom_data();

        let request = ReportDERControlRequest::new(999)
            .with_curve(curve_data.clone())
            .with_enter_service(enter_service_data.clone())
            .with_fixed_pf_absorb(fixed_pf_absorb_data.clone())
            .with_fixed_pf_inject(fixed_pf_inject_data.clone())
            .with_fixed_var(fixed_var_data.clone())
            .with_freq_droop(freq_droop_data.clone())
            .with_gradient(gradient_data.clone())
            .with_limit_max_discharge(limit_max_discharge_data.clone())
            .with_tbc(true)
            .with_custom_data(custom_data.clone());

        // Test all getters
        assert_eq!(*request.get_request_id(), 999);
        assert_eq!(request.get_curve(), Some(&curve_data));
        assert_eq!(request.get_enter_service(), Some(&enter_service_data));
        assert_eq!(request.get_fixed_pf_absorb(), Some(&fixed_pf_absorb_data));
        assert_eq!(request.get_fixed_pf_inject(), Some(&fixed_pf_inject_data));
        assert_eq!(request.get_fixed_var(), Some(&fixed_var_data));
        assert_eq!(request.get_freq_droop(), Some(&freq_droop_data));
        assert_eq!(request.get_gradient(), Some(&gradient_data));
        assert_eq!(request.get_limit_max_discharge(), Some(&limit_max_discharge_data));
        assert_eq!(request.get_tbc(), Some(&true));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }
}