use diesel::prelude::*;
use chrono::NaiveDateTime;

use crate::{domain::value_object::quest_model::QuestModel, infrastructure::postgres::schema::quests};

use super::adventurers;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
//Connect with Tables by set macro
#[diesel(table_name = quests)]
pub struct QuestEntity {
    pub id:i32,
    pub name:String,
    pub description: Option<String>,
    pub status: String,
    pub guild_commander_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl QuestEntity {
    pub fn to_model(&self, adventurers_count: i64) -> QuestModel {
        QuestModel { 
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(), 
            status: self.status.clone(), 
            guild_commander_id: self.guild_commander_id, 
            adventurers_count, 
            created_at: self.created_at, 
            updated_at: self.updated_at
        }
    }
}

#[derive(Debug, Clone, Insertable, Queryable)]
//Connect with Tables by set macro
#[diesel(table_name = quests)]
pub struct AddQuestEntity {
    pub name:String,
    pub description: Option<String>,
    pub status: String,
    pub guild_commander_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, AsChangeset, Queryable)]
//Connect with Tables by set macro
#[diesel(table_name = quests)]
pub struct EditQuestEntity {
    pub name:Option<String>,
    pub description: Option<String>,
    pub guild_commander_id: i32,
    pub updated_at: NaiveDateTime,
}