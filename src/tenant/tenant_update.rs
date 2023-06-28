#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TenantSbStatus {
    NotInitialized = 0,
    Initializing = 1,
    Initialized = 2,
    Deploying = 3,
    ConfigurationEnv = 4,
    Active = 5,
    Removing = 6,
    Removed = 7,
    Failed = 8,
}

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "tenant-status-update")]
pub struct TenantUpdateSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(enumeration = "TenantSbStatus", tag = "2")]
    pub status: i32,
    #[prost(string, tag = "3")]
    pub process_id: String,
    #[prost(uint64, tag = "4")]
    pub date_time: u64,
}
