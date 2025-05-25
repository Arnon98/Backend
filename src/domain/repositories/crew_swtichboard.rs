use axum::async_trait;
use mockall::automock;
use anyhow::Result;

use crate::domain::value_object::quest_adventurer_junction::QuestAdventurerJunction;

#[async_trait]
#[automock]
pub trait CrewSwitchBoardRepository {
    async fn join (&self, junction_body: QuestAdventurerJunction) -> Result<()>;
    async fn leave (&self, junction_body: QuestAdventurerJunction) -> Result<()>;
}