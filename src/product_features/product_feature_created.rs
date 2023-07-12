pub struct ProductFeatureEnabledSbModel{
    pub process_id: String,
    pub tenant_id: String,
    pub product_id: String,
    pub product_instance_id: String,
    pub feature_id: String,
    pub feature_instance_id: String,
}

pub struct ProductFeatureDeployedSbModel{
    pub tenant_id: String,
    pub product_id: String,
    pub product_instance_id: String,
    pub feature_id: String,
    pub feature_instance_id: String,
}