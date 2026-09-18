use crate::features::machines::domain::{ClawSize, Machine, MachineStatus};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PostgresMachineRepository {
    pool: PgPool,
}

impl PostgresMachineRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, machine: &Machine) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
                INSERT INTO machines (
                    id,
                    model,
                    claw_size,
                    capacity_toys,
                    cost_machine,
                    status_machine,
                    coin_acceptors,
                    bill_acceptor,
                    plays_per_plush,
                    credit_cost_per_play,
                    img_url,
                    purchase_date
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(machine.id)
        .bind(&machine.model)
        .bind(&machine.claw_size)
        .bind(machine.capacity_toys)
        .bind(machine.cost_machine)
        .bind(&machine.status_machine)
        .bind(machine.coin_acceptors)
        .bind(machine.bill_acceptor)
        .bind(machine.plays_per_plush)
        .bind(machine.credit_cost_per_play)
        .bind(&machine.img_url)
        .bind(machine.purchase_date)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<Machine>, sqlx::Error> {
        let machines = sqlx::query_as!(
            Machine,
            r#"
        SELECT 
            id,
            model,
            claw_size AS "claw_size: ClawSize",
            capacity_toys,
            cost_machine,
            status_machine AS "status_machine: MachineStatus",
            coin_acceptors,
            bill_acceptor,
            plays_per_plush,
            credit_cost_per_play,
            img_url,
            purchase_date
        FROM machines
        "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(machines)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Machine, sqlx::Error> {
        let machine = sqlx::query_as!(
            Machine,
            r#"
            SELECT 
                id,
                model,
                claw_size AS "claw_size: ClawSize",
                capacity_toys,
                cost_machine,
                status_machine AS "status_machine: MachineStatus",
                coin_acceptors,
                bill_acceptor,
                plays_per_plush,
                credit_cost_per_play,
                img_url,
                purchase_date
            FROM machines
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(machine)
    }
}
