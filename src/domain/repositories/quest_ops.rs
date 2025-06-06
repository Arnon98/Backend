use axum::async_trait;
use mockall::automock;
use anyhow::Result;

use crate::domain::entities::quests::{AddQuestEntity, EditQuestEntity};

#[async_trait]
#[automock]

pub trait QuestOpsRepository {
    async fn add(&self, add_quest_entity: AddQuestEntity) -> Result<i32>;
    async fn edit(&self, quest_id:i32, edit_quest_entity: EditQuestEntity) -> Result<i32>;
    async fn remove(&self, quest_id:i32, guild_commander:i32) ->Result<()>;
}