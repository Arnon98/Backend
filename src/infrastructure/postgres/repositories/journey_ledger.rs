use std::sync::Arc;
use axum::async_trait;
use anyhow::Result;

use crate::{domain::repositories::journey_ledger::JourneyLedegerRepository, infrastructure::postgres::postgres_connection::PgPoolSquad};


pub struct JourneyLedegerPostgres {
    db_pool: Arc<PgPoolSquad>
}

impl JourneyLedegerPostgres {
    pub fn new (db_pool: Arc<PgPoolSquad>) -> Self {
        Self {db_pool}
    }
}

#[async_trait]
impl JourneyLedegerRepository for JourneyLedegerPostgres  {    
    async fn in_journey(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32>{
        unimplemented!();
    }
    async fn to_compleatd(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        unimplemented!();
    }
    async fn to_failed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        unimplemented!();
    }
}