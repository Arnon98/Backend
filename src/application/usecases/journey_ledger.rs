use std::sync::Arc;

use anyhow::Result;

use crate::domain::repositories::{journey_ledger::JourneyLedegerRepository, quest_viewing::QuestViewingRepository};

pub struct JourneyLedgerUseCase<T1,T2>
where 
    T1: JourneyLedegerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    journey_ledger_repository: Arc<T1>,
    quest_viewing_repository: Arc<T2>,
}

impl<T1, T2> JourneyLedgerUseCase<T1, T2>
where 
    T1: JourneyLedegerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub fn new(journey_ledger_repository: Arc<T1>, quest_viewing_repository: Arc<T2>) -> Self {
        Self {
            journey_ledger_repository,
            quest_viewing_repository,
        }
    }

    pub async fn in_journey(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32>{
        unimplemented!()
    }

        pub async fn to_compleatd_completed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32>{
        unimplemented!()
    }

        pub async fn to_failed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32>{
        unimplemented!()
    }
}