use std::collections::HashSet;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, SingleInsert};

use super::device_models::DeviceModelsDatabaseTable;
use super::parts::PartsDatabaseTable;
use super::IdentifiableRow;
use crate::database::{DatabaseEntity, GenerateRowData};

#[derive(DatabaseEntity, BulkInsert, GenerateTableData)]
#[entity(entity_name = "compatible_parts", primary_key = "(device, part)")]
pub struct CompatiblePartsDatabaseJunctionTable {
    rows: Vec<CompatiblePartsDatabaseJunctionTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, Clone)]
pub struct CompatiblePartsDatabaseJunctionTableRow {
    pub device: i32,
    pub part: i32,
}

impl GenerateRowData for CompatiblePartsDatabaseJunctionTableRow {
    type Identifier = (i32, i32);
    type Dependencies<'a> = (&'a DeviceModelsDatabaseTable, &'a PartsDatabaseTable);
    fn generate(
        existing_pairs: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let mut device_id = 0;
        let mut part_id = 0;
        let mut first_roll = true;
        while first_roll || existing_pairs.get(&(device_id, part_id)).is_some() {
            device_id = dependencies.0.pick_random().id();
            part_id = dependencies.1.pick_random().id();
            first_roll = false;
        }

        existing_pairs.insert((device_id, part_id));

        Self {
            device: device_id,
            part: part_id,
        }
    }
}
