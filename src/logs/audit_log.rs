#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "audit-log")]
pub struct CloudAuditLogModel {
    #[prost(string, tag = "1")]
    pub process_id: String,
    #[prost(uint64, tag = "2")]
    pub date_time: u64,
    #[prost(string, tag = "3")]
    pub message: String,
    #[prost(message, tag = "4")]
    pub obj: Option<String>,
    #[prost(message, tag = "5")]
    pub context: Option<String>,
    #[prost(message, repeated, tag = "6")]
    pub labels: Vec<CloudAuditLogLabel>,
    #[prost(string, tag = "7")]
    pub index: String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudAuditLogLabel {
    #[prost(string, tag = "1")]
    pub key: String,
    #[prost(string, tag = "2")]
    pub value: String,
}
