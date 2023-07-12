#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "add-tenant-product")]
pub struct AddTenantProductSbModel {
    #[prost(string, tag = "1")]
    pub process_id: String,
    #[prost(string, tag = "2")]
    pub tenant_id: String,
    #[prost(string, tag = "3")]
    pub product_id: String,
    #[prost(string, tag = "4")]
    pub product_instance_id: String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "tenant-product-added")]
pub struct TenantProductAddedSbModel {
    #[prost(string, tag = "1")]
    pub tenant_id: String,
    #[prost(string, tag = "2")]
    pub product_id: String,
    #[prost(string, tag = "3")]
    pub product_instance_id: String,
    #[prost(string, tag = "4")]
    pub ip: String,
}
