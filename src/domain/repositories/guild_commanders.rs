use axum::async_trait;
use mockall::automock;
use anyhow::Result;
use crate::domain::{entities::guild_commanders::GuildCommanderEntity, 
    value_object::guild_commander_model::RegisterGuildCommanderModel};

#[async_trait]
#[automock]
pub trait GuildCommanderRepository {
    async fn register(&self, register_guild_commander_model:RegisterGuildCommanderModel) -> Result<i32>;
    async fn find_by_username(&self, username:String) -> Result<GuildCommanderEntity>;
}