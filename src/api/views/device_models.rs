use serde::Serialize;

use super::{
    ColumnFormat, FrontendColumnDisplay, FrontendColumnMetadata, FrontendDataType, ViewCell,
};
use crate::api::{DatabaseEntity, FromDatabaseEntity};
use crate::database::views::device_models::{
    DeviceModelsDatabaseView, DeviceModelsDatabaseViewRow,
};

#[derive(Serialize)]
pub struct DeviceModelsApiView {
    metadata: DeviceModelsApiViewMetadata,
    rows: Vec<DeviceModelsApiViewRow>,
}

#[derive(Serialize)]
struct DeviceModelsApiViewRow {
    id: ViewCell<u32>,
    display_name: ViewCell<String>,
    manufacturer: ViewCell<String>,
    category: ViewCell<String>,
}

struct DeviceModelsApiViewFormatting {
    id: ColumnFormat,
    display_name: ColumnFormat,
    manufacturer: ColumnFormat,
    category: ColumnFormat,
}

#[derive(Serialize)]
struct DeviceModelsApiViewMetadata {
    id: FrontendColumnMetadata,
    display_name: FrontendColumnMetadata,
    manufacturer: FrontendColumnMetadata,
    category: FrontendColumnMetadata,
}

impl DeviceModelsApiViewFormatting {
    const fn new() -> Self {
        Self {
            id: ColumnFormat::Id,
            display_name: ColumnFormat::None,
            manufacturer: ColumnFormat::None,
            category: ColumnFormat::None,
        }
    }
}

impl DeviceModelsApiViewMetadata {
    const fn new() -> Self {
        Self {
            id: FrontendColumnMetadata {
                data_type: FrontendDataType::Integer,
                display: FrontendColumnDisplay::Text {
                    name: "ID",
                    trimmable: false,
                },
            },
            display_name: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Name",
                    trimmable: false,
                },
            },
            manufacturer: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Manufacturer",
                    trimmable: false,
                },
            },
            category: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Category",
                    trimmable: false,
                },
            },
        }
    }
}

impl FromDatabaseEntity for DeviceModelsApiView {
    type Entity = DeviceModelsDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        let formatting = DeviceModelsApiViewFormatting::new();
        Self {
            metadata: DeviceModelsApiViewMetadata::new(),
            rows: entity
                .take_rows()
                .into_iter()
                .map(|row| {
                    let DeviceModelsDatabaseViewRow {
                        id,
                        display_name,
                        manufacturer,
                        category,
                    } = row;

                    DeviceModelsApiViewRow {
                        id: ViewCell::new(id as u32, &formatting.id),
                        display_name: ViewCell::new(display_name, &formatting.display_name),
                        manufacturer: ViewCell::new(manufacturer, &formatting.manufacturer),
                        category: ViewCell::new(category, &formatting.category),
                    }
                })
                .collect(),
        }
    }
}
