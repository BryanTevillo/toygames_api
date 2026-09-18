mod features;
mod shared;

use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use features::machines::application::use_cases::{CreateMachineCommand, CreateMachineUseCase};
use features::machines::domain::{ClawSize, Machine, MachineStatus};
use features::machines::{
    infrastructure::controller::{
        create_machine_handler, get_all_machines_handler, get_machine_by_id_handler,
    },
    infrastructure::repository::PostgresMachineRepository,
};
use shared::db::create_pool;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let machine = Machine {
        id: Uuid::new_v4(),
        model: "Machine 5".to_string(),
        claw_size: ClawSize::Small,
        capacity_toys: 65,
        cost_machine: 32500.60,
        status_machine: MachineStatus::Warehouse,
        coin_acceptors: 2,
        bill_acceptor: 1,
        plays_per_plush: Some(1),
        credit_cost_per_play: Some(5),
        img_url: Some("http://example.com".to_string()),
        purchase_date: Some(chrono::NaiveDate::from_ymd_opt(2026, 09, 30).unwrap()),
    };

    println!("{:?}", machine);

    let json_string = serde_json::to_string_pretty(&machine).unwrap();
    println!("--Objeto serializado osea a JSON --\n{} ", json_string);

    let decode_machine: Machine = serde_json::from_str(&json_string).unwrap();
    println!(
        "--Objeto deserializado osea de JSON a Machine --\n{:?} ",
        decode_machine
    );

    let database_url = "postgresql://postgres:postgres@localhost:5433/toygames_db";
    let pool = create_pool(database_url).await?;

    let repo = Arc::new(PostgresMachineRepository::new(pool));

    println!("Conexion a base de datos exitosa");
    let usecase = CreateMachineUseCase::new(&repo);

    let command = CreateMachineCommand {
        model: "New Model 10".to_string(),
        claw_size: ClawSize::Large,
        capacity_toys: 65,
        cost_machine: 29865.0,
        status_machine: MachineStatus::Work,
        coin_acceptors: 2,
        bill_acceptor: 1,
        plays_per_plush: Some(30),
        credit_cost_per_play: Some(5),
        img_url: Some("http://example.com/2".to_string()),
        purchase_date: Some(chrono::NaiveDate::from_ymd_opt(2025, 05, 10).unwrap()),
    };

    match usecase.execute(command).await {
        Ok(id) => println!("Machine created successfully con ID: {}", id),
        Err(e) => println!("Error creating machine: {}", e),
    }

    let app: Router = Router::new()
        .route("/machines", post(create_machine_handler))
        .route("/machines", get(get_all_machines_handler))
        .route("/machine/{id}", get(get_machine_by_id_handler))
        .with_state(repo);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("Servidor corriendo en http://127.0.0.1:3000");

    axum::serve(listener, app).await?;
    Ok(())
}
