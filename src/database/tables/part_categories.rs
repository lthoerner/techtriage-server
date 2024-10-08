use proc_macros::{BulkInsert, IdentifiableRecord, Relation, SingleInsert, Table};

use crate::database::{GenerateStaticRecord, GenerateStaticTable};

#[derive(Relation, Table, BulkInsert, Clone)]
#[relation(relation_name = "part_categories", primary_key = "id")]
pub struct PartCategoriesTable {
    records: Vec<PartCategoriesTableRecord>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone)]
pub struct PartCategoriesTableRecord {
    pub id: i32,
    pub display_name: String,
}

impl GenerateStaticTable for PartCategoriesTable {
    const ITEMS: &[&str] = &[
        "Screen",
        "Battery",
        "Backglass",
        "Frame",
        "Front Camera",
        "Rear Camera",
        "Charge Port",
    ];
}

impl GenerateStaticRecord for PartCategoriesTableRecord {
    fn new(id: i32, display_name: impl Into<String>) -> Self {
        Self {
            id,
            display_name: display_name.into(),
        }
    }
}
