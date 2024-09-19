use serde::Serialize;

use proc_macros::{ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::{FromDatabaseEntity, FromDatabaseRow, GenericIdParameter};
use crate::database::views::vendors::{VendorsDatabaseView, VendorsDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(ServeEntityJson, Serialize)]
pub struct VendorsApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<VendorsApiEndpointRow>,
}

#[derive(ProcessEndpoint, ServeRowJson, Serialize)]
#[id_param(GenericIdParameter)]
pub struct VendorsApiEndpointRow {
    #[col_format(
        format = "id",
        data_type = "integer",
        display_name = "ID",
        trimmable = false
    )]
    id: ViewCell<u32>,
    #[col_format(data_type = "string", display_name = "Name", trimmable = false)]
    display_name: ViewCell<String>,
}

impl FromDatabaseEntity for VendorsApiEndpoint {
    type Entity = VendorsDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        Self {
            metadata: EndpointMetadata::new(),
            rows: entity
                .take_rows()
                .into_iter()
                .map(VendorsApiEndpointRow::from_database_row)
                .collect(),
        }
    }
}

impl FromDatabaseRow for VendorsApiEndpointRow {
    type Row = VendorsDatabaseViewRow;
    fn from_database_row(row: Self::Row) -> Self {
        let formatting = EndpointFormatting::new();
        let VendorsDatabaseViewRow { id, display_name } = row;
        VendorsApiEndpointRow {
            id: ViewCell::new(id as u32, &formatting.id),
            display_name: ViewCell::new(display_name, &formatting.display_name),
        }
    }
}
