use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::Relation;

use crate::database::shared_models::TicketStatus;

#[derive(Relation, Serialize)]
#[relation(relation_name = "tickets_view", primary_key = "id")]
pub struct TicketsView {
    records: Vec<TicketsViewRecord>,
}

#[derive(sqlx::FromRow, Serialize, Clone)]
pub struct TicketsViewRecord {
    pub id: i32,
    pub status: TicketStatus,
    pub customer: Option<String>,
    pub balance: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
