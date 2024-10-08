use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::Relation;

#[derive(Relation, Serialize)]
#[relation(relation_name = "parts_view", primary_key = "id")]
pub struct PartsView {
    records: Vec<PartsViewRecord>,
}

#[derive(sqlx::FromRow, Serialize, Clone)]
pub struct PartsViewRecord {
    pub id: i32,
    pub display_name: String,
    pub vendor: String,
    pub manufacturer: Option<String>,
    pub category: String,
    pub cost: Option<Decimal>,
    pub price: Option<Decimal>,
}
