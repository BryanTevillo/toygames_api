use crate::features::machines::{
    application::use_cases::{CreateMachineCommand, CreateMachineUseCase},
    domain::{ClawSize, MachineStatus},
    infrastructure::repository::PostgresMachineRepository,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::NaiveDate;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateMachineRequest {
    pub model: String,
    pub claw_size: ClawSize,
    pub capacity_toys: i16,
    pub cost_machine: f32,
    pub status_machine: MachineStatus,
    pub coin_acceptors: i16,
    pub bill_acceptor: i16,
    pub plays_per_plush: Option<i16>,
    pub credit_cost_per_play: Option<i16>,
    pub img_url: Option<String>,
    pub purchase_date: Option<NaiveDate>,
}

pub async fn create_machine_handler(
    State(repo): State<Arc<PostgresMachineRepository>>,
    Json(payload): Json<CreateMachineRequest>,
) -> impl IntoResponse {
    let use_case = CreateMachineUseCase::new(&repo);

    let command = CreateMachineCommand {
        model: payload.model,
        claw_size: payload.claw_size,
        capacity_toys: payload.capacity_toys,
        cost_machine: payload.cost_machine,
        status_machine: payload.status_machine,
        coin_acceptors: payload.coin_acceptors,
        bill_acceptor: payload.bill_acceptor,
        plays_per_plush: payload.plays_per_plush,
        credit_cost_per_play: payload.credit_cost_per_play,
        img_url: payload.img_url,
        purchase_date: payload.purchase_date,
    };

    match use_case.execute(command).await {
        Ok(id) => (StatusCode::CREATED, Json(serde_json::json!({"id": id}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

pub async fn get_all_machines_handler(
    State(repo): State<Arc<PostgresMachineRepository>>,
) -> impl IntoResponse {
    match repo.find_all().await {
        Ok(machines) => (StatusCode::OK, Json(machines)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

pub async fn get_machine_by_id_handler(
    State(repo): State<Arc<PostgresMachineRepository>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.find_by_id(id).await {
        Ok(machine) => (StatusCode::OK, Json(machine)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
