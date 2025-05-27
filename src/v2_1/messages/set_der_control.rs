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
