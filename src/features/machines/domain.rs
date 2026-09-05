use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Type, Deserialize, Serialize)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum MachineStatus {
    Work,
    Damage,
    Warehouse,
}

#[derive(Debug, Type, Deserialize, Serialize)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ClawSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[allow(dead_code)] //solo para quitar las alertas y trabajar quitar una vez que se lean individualmente
pub struct Machine {
    pub id: Uuid,                          // identificador o codigo (NOT NULL)
    pub model: String,                     // modelo (NOT NULL)
    pub claw_size: ClawSize,               // tamaño de la garra (NOT NULL)
    pub capacity_toys: i16,                // capacidad de juguetes (NOT NULL)
    pub cost_machine: f32,                 // costo de la maquina (NOT NULL)
    pub status_machine: MachineStatus,     // estado de la maquina (NOT NULL)
    pub coin_acceptors: i16,               // numero de aceptadores de monedas (NOT NULL)
    pub bill_acceptor: i16,                // numero de aceptadores de billetes (NOT NULL)
    pub plays_per_plush: Option<u8>,       // numero de jugadas por peluche (NULL)
    pub credit_cost_per_play: Option<f64>, // costo por jugada (NULL)
    pub img_url: Option<String>,           // url de la imagen (NULL)
    pub purchase_date: Option<NaiveDate>,  // fecha de compra (NULL)
}
