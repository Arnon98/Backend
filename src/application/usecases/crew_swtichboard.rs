use std::sync::Arc;
use anyhow::Result;
use crate::{domain::repositories::{crew_swtichboard::CrewSwitchBoardRepository, quest_viewing::QuestViewingRepository}};
pub struct CrewSwitchBoardUsecase<T1,T2>
where 
    T1: CrewSwitchBoardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    
    crew_switchboard_repository: Arc<T1>,
    quest_viewing_repository: Arc<T2>,
}  

impl<T1, T2> CrewSwitchBoardUsecase<T1, T2>
where 
    T1: CrewSwitchBoardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub fn new (crew_switchboard_repository:Arc<T1>,quest_viewing_repository: Arc<T2>) -> Self 
    {
        Self
        {
            quest_viewing_repository,
            crew_switchboard_repository,
        }
    }

    pub async fn join(&self, quest_id: i32, adventurer_id: i32) -> Result<()> 
    {   
        unimplemented!()
    }
    pub async fn leave(&self, quest_id: i32, adventurer_id: i32) -> Result<()>
    {
        unimplemented!()
    }
}