use axum::async_trait;
use mockall::automock;
use anyhow::Result;

#[async_trait]
#[automock]
pub trait JourneyLedegerRepository {
    async fn in_journey(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32>;
    async fn to_compleatd(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32>;
    async fn to_failed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32>;
}
    
