use std::sync::Arc;
use anyhow::Result;
use axum::async_trait;

use crate::{domain::{repositories::crew_swtichboard::CrewSwitchBoardRepository, value_object::quest_adventurer_junction::QuestAdventurerJunction}, infrastructure::postgres::postgres_connection::PgPoolSquad};

pub struct CrewSwitchBoardPostgres {
    db_pool:Arc<PgPoolSquad>
}

impl CrewSwitchBoardPostgres {
    pub fn new (db_pool: Arc<PgPoolSquad>) -> Self {
        Self {db_pool}
    }
}

#[async_trait]
impl CrewSwitchBoardRepository for CrewSwitchBoardPostgres {
    async fn join (&self, junction_body: QuestAdventurerJunction) -> Result<()>{
        unimplemented!();
    }
    async fn leave (&self, junction_body: QuestAdventurerJunction) -> Result<()>{
        unimplemented!();
    }
}