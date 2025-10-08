#[tauri::command]
fn get_all_cards() -> Result<Vec<CardDto>, String> {
    use crate::schema::cards::dsl::*;
    let mut conn = establish_connection();
    let all_cards = cards.load::<Card>(&mut conn).map_err(|e| format!("DB error: {}", e))?;
    Ok(all_cards.into_iter().map(|c| c.into()).collect())
}

#[tauri::command(rename_all = "camelCase")]
fn update_card_info(cardId: i32, cardName: Option<String>, userFullname: Option<String>, userId: Option<String>) -> Result<(), String> {
    use crate::schema::cards::dsl::*;
    let mut conn = establish_connection();
    diesel::update(cards.filter(id.eq(cardId)))
        .set((
            card_name.eq(cardName),
            user_fullname.eq(userFullname),
            user_id.eq(userId),
            updated_at.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(&mut conn)
        .map_err(|e| format!("Update error: {}", e))?;
    Ok(())
}
#[tauri::command]
fn clear_punch_logs() -> Result<(), String> {
    use crate::schema::punches::dsl::*;
    let mut conn = establish_connection();
    diesel::delete(punches)
        .execute(&mut conn)
        .map_err(|e| format!("Failed to clear logs: {}", e))?;
    Ok(())
}
#[derive(serde::Serialize)]
pub struct PunchLogDto {
    pub id: i32,
    pub card_number: String,
    pub card_name: Option<String>,
    pub user_fullname: Option<String>,
    pub status: String,
    pub timestamp: String,
}

#[tauri::command]
fn get_punch_log() -> Result<Vec<PunchLogDto>, String> {
    use crate::schema::punches::dsl::*;
    use crate::schema::cards::dsl as cards_dsl;
    let mut conn = establish_connection();
    let results = punches
        .inner_join(cards_dsl::cards)
        .order(timestamp.desc())
        .load::<(models::Punch, models::Card)>(&mut conn)
        .map_err(|e| format!("DB error: {}", e))?;
    Ok(results
        .into_iter()
        .map(|(punch, card)| PunchLogDto {
            id: punch.id,
            card_number: card.card_number,
            card_name: card.card_name,
            user_fullname: card.user_fullname,
            status: punch.status,
            timestamp: chrono::DateTime::<chrono::Utc>::from_utc(punch.timestamp, chrono::Utc)
                .with_timezone(&chrono_tz::Europe::Ljubljana)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        })
        .collect())
}
#[derive(serde::Serialize)]
pub struct CardDto {
    pub id: i32,
    pub card_number: String,
    pub card_name: Option<String>,
    pub user_fullname: Option<String>,
    pub user_id: Option<String>,
    pub is_present: bool,
}

impl From<Card> for CardDto {
    fn from(card: Card) -> Self {
        CardDto {
            id: card.id,
            card_number: card.card_number,
            card_name: card.card_name,
            user_fullname: card.user_fullname,
            user_id: card.user_id,
            is_present: card.is_present,
        }
    }
}
use chrono::Utc;
use diesel::prelude::*;
use models::{Card, NewCard};


#[tauri::command]
fn find_or_create_user(card_uid: String, _user_fullname: Option<String>) -> Result<CardDto, String> {
    use crate::schema::cards::dsl::*;
    let mut conn = establish_connection();
    match cards.filter(card_number.eq(&card_uid)).first::<Card>(&mut conn) {
        Ok(card) => Ok(card.into()),
        Err(diesel::result::Error::NotFound) => {
            let new_card = NewCard {
                card_number: &card_uid,
                card_name: None,
            };
            diesel::insert_into(cards)
                .values(&new_card)
                .execute(&mut conn)
                .map_err(|e| format!("Insert error: {}", e))?;
            cards.filter(card_number.eq(&card_uid)).first::<Card>(&mut conn).map(|c| c.into()).map_err(|e| format!("Fetch error: {}", e))
        }
        Err(e) => Err(format!("DB error: {}", e)),
    }
}

#[tauri::command]
fn register_leave(card_uid: String, leave_type: String) -> Result<String, String> {
    use crate::schema::cards::dsl::*;
    use crate::schema::punches::dsl as punches_dsl;
    let mut conn = establish_connection();
    let now = Utc::now().naive_utc();
    let card = cards.filter(card_number.eq(&card_uid)).first::<Card>(&mut conn).map_err(|_| "Card not found".to_string())?;
    let updated = diesel::update(cards.filter(card_number.eq(&card_uid)))
        .set((is_present.eq(true), updated_at.eq(now)))
        .execute(&mut conn)
        .map_err(|e| format!("Update error: {}", e))?;
    use diesel::prelude::*;
    let last_punch = punches_dsl::punches
        .filter(punches_dsl::card_id.eq(card.id))
        .order(punches_dsl::timestamp.desc())
        .first::<models::Punch>(&mut conn)
        .ok();
    let should_log = match last_punch {
        Some(ref punch) => {
            let diff = now.signed_duration_since(punch.timestamp).num_seconds();
            punch.status != leave_type || diff > 1
        },
        None => true,
    };
    if should_log {
        let new_punch = models::NewPunch {
            card_id: card.id,
            status: &leave_type,
        };
        diesel::insert_into(punches_dsl::punches)
            .values(&new_punch)
            .execute(&mut conn)
            .map_err(|e| format!("Punch log error: {}", e))?;
    }
    Ok(format!("Leave '{}' registered for card {}", leave_type, card_uid))
}
pub mod models;
pub mod schema;

use app_dirs2::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

const APP_INFO: AppInfo = AppInfo {
    name: "Terminal",
    author: "UrnikNET",
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection() -> SqliteConnection {
    let app_dir =
        app_root(AppDataType::UserConfig, &APP_INFO).expect("Failed to get app root directory");

    let database_path = app_dir.join("terminal.db");
    let database_url = database_path
        .to_str()
        .expect("Failed to convert path to string");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_pending_migrations(connection: &mut SqliteConnection) {
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_card_uid(_timeout: Option<i32>, is_leave: Option<bool>, uid: Option<Vec<u8>>, leave_type: Option<String>) -> Result<Vec<u8>, String> {
    let uid = uid.unwrap_or_else(|| vec![0xAB, 0xBA, 0xDE, 0xDA]);
    let uid_hex = uid.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(":");
    println!("read_card_uid called with UID: {}", uid_hex);
    use crate::schema::cards::dsl::*;
    use crate::schema::punches::dsl as punches_dsl;
    let mut conn = establish_connection();

    if let Ok(card) = cards.filter(card_number.eq(&uid_hex)).first::<Card>(&mut conn) {
        use diesel::prelude::*;
        use chrono::NaiveDateTime;
        let last_punch = punches_dsl::punches
            .filter(punches_dsl::card_id.eq(card.id))
            .order(punches_dsl::timestamp.desc())
            .first::<models::Punch>(&mut conn)
            .ok();
        let now = chrono::Utc::now().naive_utc();
        let should_log = match last_punch {
            Some(ref punch) => {
                let diff = now.signed_duration_since(punch.timestamp).num_seconds();
                diff > 1
            },
            None => true,
        };
        if should_log {
            let is_leave_scan = is_leave.unwrap_or(false);
            if is_leave_scan {
                let status_str = leave_type.as_deref().unwrap_or("leave_type.unknown");
                let new_punch = models::NewPunch {
                    card_id: card.id,
                    status: status_str,
                };
                let _ = diesel::insert_into(punches_dsl::punches)
                    .values(&new_punch)
                    .execute(&mut conn);
            } else {
                let should_punch_in = match &last_punch {
                    None => true,
                    Some(p) => p.status != "In",
                };
                if should_punch_in {
                    let new_punch = models::NewPunch {
                        card_id: card.id,
                        status: "In",
                    };
                    let _ = diesel::insert_into(punches_dsl::punches)
                        .values(&new_punch)
                        .execute(&mut conn);
                }
            }
        }
    }
    Ok(uid)
}

#[tauri::command]
fn punch_out(card_uid: String) -> Result<String, String> {
    use crate::schema::cards::dsl::*;
    use crate::schema::punches::dsl as punches_dsl;
    let mut conn = establish_connection();
    let now = Utc::now().naive_utc();
    let card = cards.filter(card_number.eq(&card_uid)).first::<Card>(&mut conn).map_err(|_| "Card not found".to_string())?;
    
    diesel::update(cards.filter(card_number.eq(&card_uid)))
        .set((is_present.eq(false), updated_at.eq(now)))
        .execute(&mut conn)
        .map_err(|e| format!("Update error: {}", e))?;
    
    let new_punch = models::NewPunch {
        card_id: card.id,
        status: "Out",
    };
    diesel::insert_into(punches_dsl::punches)
        .values(&new_punch)
        .execute(&mut conn)
        .map_err(|e| format!("Punch log error: {}", e))?;
    Ok("Punch out registered".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![greet, read_card_uid, find_or_create_user, register_leave, punch_out, get_punch_overview, get_punch_log, clear_punch_logs, get_all_cards, update_card_info, delete_card])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_punch_overview() -> Result<Vec<CardDto>, String> {
    use crate::schema::cards::dsl::*;
    let mut conn = establish_connection();
    let all_cards = cards.load::<Card>(&mut conn).map_err(|e| format!("DB error: {}", e))?;
    Ok(all_cards.into_iter().map(|c| c.into()).collect())
}

#[tauri::command]
fn delete_card(card_id: i32) -> Result<(), String> {
    use crate::schema::cards::dsl::{cards, id as card_id_col};
    use crate::schema::punches::dsl::{punches, card_id as punches_card_id};
    let mut conn = establish_connection();
    let _ = diesel::delete(punches.filter(punches_card_id.eq(card_id))).execute(&mut conn);

    diesel::delete(cards.filter(card_id_col.eq(card_id)))
        .execute(&mut conn)
        .map_err(|e| format!("Delete error: {}", e))?;
    Ok(())
}