
#[derive(Queryable, serde::Serialize)]
pub struct Punch {
    pub id: i32,
    pub card_id: i32,
    pub status: String,
    pub timestamp: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::punches)]
pub struct NewPunch<'a> {
    pub card_id: i32,
    pub status: &'a str,
}
use diesel::prelude::*;

#[derive(Queryable, Selectable, serde::Serialize)]
#[diesel(table_name = crate::schema::cards)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Card {
    pub id: i32,
    pub card_number: String,
    pub card_name: Option<String>,
    pub user_fullname: Option<String>,
    pub user_id: Option<String>,
    pub is_present: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::cards)]
pub struct NewCard<'a> {
    pub card_number: &'a str,
    pub card_name: Option<&'a str>,
}
