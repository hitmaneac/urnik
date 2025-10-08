#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use diesel::prelude::*;
use urniknet_terminal_lib::models::*;
use urniknet_terminal_lib::*;

fn main() {
    use urniknet_terminal_lib::schema::cards::dsl::*;

    let connection = &mut establish_connection();

    run_pending_migrations(connection);

    urniknet_terminal_lib::run()
}
