use crate::features::machines::domain::Machine;
use crate::features::machines::infrastructure::repository::PostgresMachineRepository;
use chrono::NaiveDate;
use uuid::Uuid;

pub struct CreateMachineCommand {
    pub model: String,
    pub claw_size: crate::features::machines::domain::ClawSize,
    pub capacity_toys: i16,
    pub cost_machine: f32,
    pub status_machine: crate::features::machines::domain::MachineStatus,
    pub coin_acceptors: i16,
    pub bill_acceptor: i16,
    pub plays_per_plush: Option<i16>,
    pub credit_cost_per_play: Option<i16>,
    pub img_url: Option<String>,
    pub purchase_date: Option<NaiveDate>,
}

pub struct CreateMachineUseCase<'a> {
    repo: &'a PostgresMachineRepository,
}

impl<'a> CreateMachineUseCase<'a> {
    pub fn new(repo: &'a PostgresMachineRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, cmd: CreateMachineCommand) -> Result<Uuid, sqlx::Error> {
        let machine_id = Uuid::new_v4();

        let machine = Machine {
            id: machine_id,
            model: cmd.model,
            claw_size: cmd.claw_size,
            capacity_toys: cmd.capacity_toys,
            cost_machine: cmd.cost_machine,
            status_machine: cmd.status_machine,
            coin_acceptors: cmd.coin_acceptors,
            bill_acceptor: cmd.bill_acceptor,
            plays_per_plush: cmd.plays_per_plush,
            credit_cost_per_play: cmd.credit_cost_per_play,
            img_url: cmd.img_url,
            purchase_date: cmd.purchase_date,
        };

        self.repo.save(&machine).await?;

        Ok(machine.id)
    }
}
