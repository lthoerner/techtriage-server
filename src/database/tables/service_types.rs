use proc_macros::{BulkInsert, Relation, IdentifiableRecord, SingleInsert};

use crate::database::{GenerateStaticRecord, GenerateStaticRelation};

#[derive(Relation, BulkInsert, Clone)]
#[relation(
    relation_name = "service_types",
    primary_key = "id",
    foreign_key_name = "service_type"
)]
pub struct ServiceTypesTable {
    records: Vec<ServiceTypesTableRecord>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone)]
pub struct ServiceTypesTableRecord {
    pub id: i32,
    pub display_name: String,
}

impl GenerateStaticRelation for ServiceTypesTable {
    const ITEMS: &[&str] = &[
        "Screen Repair",
        "Battery Repair",
        "Backglass Repair",
        "Camera Repair",
        "Port Repair",
        "Other Repair",
    ];
}

impl GenerateStaticRecord for ServiceTypesTableRecord {
    fn new(id: i32, display_name: impl Into<String>) -> Self {
        Self {
            id,
            display_name: display_name.into(),
        }
    }
}
