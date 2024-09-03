use std::collections::HashSet;

use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, IdentifiableRow, SingleInsert};

use super::generators::*;
use super::part_categories::PartCategoriesDatabaseTable;
use super::part_manufacturers::PartManufacturersDatabaseTable;
use super::vendors::VendorsDatabaseTable;
use super::IdentifiableRow;
use crate::database::{DatabaseEntity, GenerateRowData};

#[derive(DatabaseEntity, BulkInsert, GenerateTableData)]
#[entity(entity_name = "parts", primary_key = "id")]
pub struct PartsDatabaseTable {
    rows: Vec<PartsDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, Clone, IdentifiableRow)]
pub struct PartsDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
    pub vendor: i32,
    pub manufacturer: Option<i32>,
    pub category: i32,
    pub cost: Option<Decimal>,
    pub price: Option<Decimal>,
}

impl GenerateRowData for PartsDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = (
        &'a VendorsDatabaseTable,
        &'a PartManufacturersDatabaseTable,
        &'a PartCategoriesDatabaseTable,
    );
    fn generate(
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let cost = generate_option(generate_dollar_value(Some(1.00), Some(500.00)), 0.8);
        let price =
            cost.map(|cost| generate_dollar_value(Some(cost.to_f32().unwrap()), Some(1000.00)));

        Self {
            id: generate_unique_i32(0, existing_ids),
            // TODO: Generate via vendor/manufacturer/category data along with compatibilities
            display_name: "PLACEHOLDER".to_owned(),
            vendor: dependencies.0.pick_random().id(),
            manufacturer: generate_option(dependencies.1.pick_random().id(), 0.2),
            category: dependencies.2.pick_random().id(),
            cost,
            price,
        }
    }
}
