#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "resource-attached")]
pub struct ResourceAttachedSbEvent {
    #[prost(string, tag = "1")]
    pub instance_id: String,
    #[prost(string, tag = "2")]
    pub resource_id: String,
    #[prost(string, tag = "3")]
    pub tenant_id: String,
}
