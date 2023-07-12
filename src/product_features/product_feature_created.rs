#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "product-feature-enabled")]
pub struct ProductFeatureEnabledSbModel {
    #[prost(string, tag = "1")]
    pub process_id: String,
    #[prost(string, tag = "2")]
    pub tenant_id: String,
    #[prost(string, tag = "3")]
    pub product_id: String,
    #[prost(string, tag = "4")]
    pub product_instance_id: String,
    #[prost(string, tag = "5")]
    pub feature_id: String,
    #[prost(string, tag = "6")]
    pub feature_instance_id: String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "product-feature-deployed")]
pub struct ProductFeatureDeployedSbModel {
    #[prost(string, tag = "1")]
    pub tenant_id: String,
    #[prost(string, tag = "2")]
    pub product_id: String,
    #[prost(string, tag = "3")]
    pub product_instance_id: String,
    #[prost(string, tag = "4")]
    pub feature_id: String,
    #[prost(string, tag = "5")]
    pub feature_instance_id: String,
}
