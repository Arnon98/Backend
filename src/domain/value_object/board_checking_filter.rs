use serde::{Deserialize, Serialize};
use super ::quest_statuses::QuestStatus;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

pub struct  BoardCheckingFilter {
    pub name: Option<String>,
    pub status:Option<QuestStatus,>,
}