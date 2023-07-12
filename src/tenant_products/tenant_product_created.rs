pub struct AddTenantProductSbModel{
    pub process_id: String,
    pub tenant_id: String,
    pub product_id: String,
    pub product_instance_id: String,
}

pub struct TenantProductAddedSbModel{
    pub tenant_id: String,
    pub product_id: String,
    pub product_instance_id: String,
    pub ip: String,
}