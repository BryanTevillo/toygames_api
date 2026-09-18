use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Type, Deserialize, Serialize)]
#[sqlx(type_name = "operation_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum OperationStatus {
    Completed,
    PendingAudit,
}

#[derive(Debug, Type, Deserialize, Serialize)]
#[sqlx(type_name = "operation_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    Refill,
    Cut,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct MachineOperation {
    pub id: Uuid,
    pub machine_id: Uuid,
    pub operation_type: OperationType,
    pub status: OperationStatus,
    pub performed_at: NaiveDate,
    pub audited_at: Option<NaiveDate>,
    pub system_coins: i16,
    pub system_prize_out: i16,
    pub lcd_img_url: String,
    pub physical_toys_count: i16,
    pub toys_added: i16,
    pub client_payout: Option<f32>,
    pub cash_withdrawn: Option<f32>,
    pub company_profit: Option<f32>,
    pub cash_discrepancy: Option<f32>,
}
