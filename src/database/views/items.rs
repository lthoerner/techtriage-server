use rust_decimal::Decimal;

use proc_macros::Relation;

use crate::database::shared_models::ItemType;

#[derive(Relation)]
#[relation(
    relation_name = "items_view",
    primary_key = "item_id",
    foreign_key_name = "PLACEHOLDER"
)]
pub struct ItemsView {
    records: Vec<ItemsViewRecord>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct ItemsViewRecord {
    pub item_id: i32,
    pub item_type: ItemType,
    pub product_sku: Option<i32>,
    pub product_name: Option<String>,
    pub product_cost: Option<Decimal>,
    pub product_price: Option<Decimal>,
    pub service_id: Option<i32>,
    pub service_type_name: Option<String>,
    pub service_device_name: Option<String>,
    pub service_base_fee: Option<Decimal>,
    pub service_labor_fee: Option<Decimal>,
}
