mod db;
mod models;
mod commands;

use commands::AppState;
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            db: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            commands::init_db,
            commands::add_game,
            commands::get_games,
            commands::add_game_record,
            commands::get_game_records,
            commands::get_hardware_types,
            commands::add_hardware,
            commands::get_hardwares,
            commands::get_consumable_types,
            commands::get_consumables,
            commands::get_medicine_categories,
            commands::get_medicines,
            commands::get_dashboard_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
