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
pub struct MachineOperation {}
