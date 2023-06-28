#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "deploy-tenant")]
pub struct DeployTenantCommand {
    #[prost(string, tag = "1")]
    pub tenant_id: String,
    #[prost(string, tag = "2")]
    pub tenant_name: String,
    #[prost(string, tag = "3")]
    pub tenant_location: String,
    #[prost(string, tag = "4")]
    pub process_id: String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "tenant-setup-failed")]
pub struct TenantCreatingFailed {
    #[prost(string, tag = "1")]
    pub tenant_id: String,
    #[prost(string, tag = "2")]
    pub tenant_name: String,
    #[prost(string, tag = "3")]
    pub process_id: String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "delete-tenant")]
pub struct TenantDeletedSbEvent {
    #[prost(string, tag = "1")]
    pub tenant_id: String,
    #[prost(string, tag = "2")]
    pub process_id: String,
    #[prost(uint64, tag = "3")]
    pub datetime: u64,
}
