use std::collections::HashSet;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, IdentifiableRow, SingleInsert};

use super::generators::*;
use crate::database::GenerateRowData;

#[derive(DatabaseEntity, BulkInsert, GenerateTableData)]
#[entity(entity_name = "customers", primary_key = "id")]
pub struct CustomersDatabaseTable {
    rows: Vec<CustomersDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, Clone, IdentifiableRow)]
pub struct CustomersDatabaseTableRow {
    pub id: i32,
    pub name: String,
    pub email_address: Option<String>,
    pub phone_number: Option<String>,
    pub street_address: Option<String>,
}

impl GenerateRowData for CustomersDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = ();
    fn generate(
        existing_ids: &mut HashSet<Self::Identifier>,
        _dependencies: Self::Dependencies<'_>,
    ) -> Self {
        Self {
            id: generate_unique_i32(0, existing_ids),
            name: generate_name(),
            email_address: generate_option(generate_email_address(), 0.9),
            phone_number: generate_option(generate_phone_number(), 0.9),
            street_address: generate_option(generate_street_address(), 0.9),
        }
    }
}
