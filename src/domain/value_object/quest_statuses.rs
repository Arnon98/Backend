use std::fmt;

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]

pub enum QuestStatus {
    #[default]
    Open,
    Injourney,
    Completed,
    Failed,
}

impl fmt::Display for QuestStatus{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        match self {
            QuestStatus::Open => write!(f, "Open"),
            QuestStatus::Injourney => write!(f,"Injourney"),
            QuestStatus::Completed => write!(f,"Completed"),
            QuestStatus::Failed => write!(f, "Failed"),
        }
    }
}