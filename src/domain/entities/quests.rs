use diesel::prelude::*;
use chrono::NaiveDateTime;

use crate::infrastructure::postgres::schema::quests;

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
    pub name:Option<Strige>,
    pub description: Option<String>,
    pub guild_commander_id: i32,
    pub updated_at: NaiveDateTime,
}