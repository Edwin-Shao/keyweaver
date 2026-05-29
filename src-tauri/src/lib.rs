mod commands;
mod engine;
mod persistence;
mod platform;
mod state;

use commands::session::EngineState;

pub fn run() {
    let engine_state = EngineState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(engine_state)
        .setup(|app| {
            platform::start_keyboard_hook(app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::session::start_lesson,
            commands::session::get_session_state,
            commands::typing::process_keystroke,
            commands::typing::typing_backspace,
            commands::stats::get_keyboard_state,
            commands::stats::get_full_stats,
            commands::settings::get_settings,
            commands::settings::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
