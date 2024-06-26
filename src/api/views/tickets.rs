use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use super::{
    ColumnFormat, CssColor, FrontendColumnDisplay, FrontendColumnMetadata, FrontendDataType,
    TagOption, ViewCell,
};
use crate::api::FromDatabaseEntity;
use crate::database::shared_models::tickets::TicketStatus;
use crate::database::views::tickets::{TicketsDatabaseView, TicketsDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(Serialize)]
pub struct TicketsApiView {
    metadata: TicketsApiViewMetadata,
    rows: Vec<TicketsApiViewRow>,
}

#[derive(Serialize)]
struct TicketsApiViewRow {
    id: ViewCell<u32>,
    status: ViewCell<TicketStatus>,
    customer: ViewCell<Option<String>>,
    balance: ViewCell<Decimal>,
    created_at: ViewCell<NaiveDateTime>,
    updated_at: ViewCell<NaiveDateTime>,
}

struct TicketsApiViewFormatting {
    id: ColumnFormat,
    status: ColumnFormat,
    customer: ColumnFormat,
    balance: ColumnFormat,
    created_at: ColumnFormat,
    updated_at: ColumnFormat,
}

#[derive(Serialize)]
struct TicketsApiViewMetadata {
    id: FrontendColumnMetadata,
    status: FrontendColumnMetadata,
    customer: FrontendColumnMetadata,
    balance: FrontendColumnMetadata,
    created_at: FrontendColumnMetadata,
    updated_at: FrontendColumnMetadata,
}

impl TicketsApiViewFormatting {
    const fn new() -> Self {
        Self {
            id: ColumnFormat::Id,
            status: ColumnFormat::Tag,
            customer: ColumnFormat::None,
            balance: ColumnFormat::Currency,
            created_at: ColumnFormat::Date,
            updated_at: ColumnFormat::Date,
        }
    }
}

impl TicketsApiViewMetadata {
    const fn new() -> Self {
        Self {
            id: FrontendColumnMetadata {
                data_type: FrontendDataType::Integer,
                display: FrontendColumnDisplay::Text {
                    name: "ID",
                    trimmable: false,
                },
            },
            status: FrontendColumnMetadata {
                data_type: FrontendDataType::Tag,
                display: FrontendColumnDisplay::Tag {
                    name: "Status",
                    options: &[
                        TagOption {
                            name: "new",
                            color: CssColor::Preset {
                                name: "royalblue",
                                opacity: 0.45,
                            },
                        },
                        TagOption {
                            name: "waiting_for_parts",
                            color: CssColor::Preset {
                                name: "red",
                                opacity: 0.37,
                            },
                        },
                        TagOption {
                            name: "waiting_for_customer",
                            color: CssColor::Preset {
                                name: "yellow",
                                opacity: 0.43,
                            },
                        },
                        TagOption {
                            name: "in_repair",
                            color: CssColor::Preset {
                                name: "orange",
                                opacity: 0.54,
                            },
                        },
                        TagOption {
                            name: "ready_for_pickup",
                            color: CssColor::Preset {
                                name: "limegreen",
                                opacity: 0.37,
                            },
                        },
                        TagOption {
                            name: "closed",
                            color: CssColor::Preset {
                                name: "gray",
                                opacity: 0.45,
                            },
                        },
                    ],
                },
            },
            customer: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Customer",
                    trimmable: true,
                },
            },
            balance: FrontendColumnMetadata {
                data_type: FrontendDataType::Decimal,
                display: FrontendColumnDisplay::Text {
                    name: "Balance",
                    trimmable: false,
                },
            },
            created_at: FrontendColumnMetadata {
                data_type: FrontendDataType::Timestamp,
                display: FrontendColumnDisplay::Text {
                    name: "Created",
                    trimmable: false,
                },
            },
            updated_at: FrontendColumnMetadata {
                data_type: FrontendDataType::Timestamp,
                display: FrontendColumnDisplay::Text {
                    name: "Updated",
                    trimmable: false,
                },
            },
        }
    }
}

impl FromDatabaseEntity for TicketsApiView {
    type Entity = TicketsDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        let formatting = TicketsApiViewFormatting::new();
        Self {
            metadata: TicketsApiViewMetadata::new(),
            rows: entity
                .take_rows()
                .into_iter()
                .map(|row| {
                    let TicketsDatabaseViewRow {
                        id,
                        status,
                        customer,
                        balance,
                        created_at,
                        updated_at,
                    } = row;

                    TicketsApiViewRow {
                        id: ViewCell::new(id as u32, &formatting.id),
                        status: ViewCell::new(status, &formatting.status),
                        customer: ViewCell::new(customer, &formatting.customer),
                        balance: ViewCell::new(balance, &formatting.balance),
                        created_at: ViewCell::new(created_at, &formatting.created_at),
                        updated_at: ViewCell::new(updated_at, &formatting.updated_at),
                    }
                })
                .collect(),
        }
    }
}
