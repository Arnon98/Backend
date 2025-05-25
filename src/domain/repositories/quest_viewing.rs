use axum::async_trait;
use mockall::automock;
use anyhow::Result;

use crate::domain::{entities::quests::QuestEntity, value_object::board_checking_filter::BoardCheckingFilter};

#[async_trait]
#[automock]

pub trait QuestViewingRepository {
    async fn view_details(&self, quest_id:i32) -> Result<QuestEntity>;
    async fn board_checking(&self, filter:&BoardCheckingFilter) -> Result<Vec<QuestEntity>>;
    async fn adventurers_counting_by_quest_id(&self, quest:i32) -> Result<i64>;
}