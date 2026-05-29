use serde::{Deserialize, Serialize};
use tauri::State;

use super::session::EngineState;
use crate::state::ErrorMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsDto {
    pub target_wpm: u32,
    pub error_mode: String,
    pub fragment_length: usize,
    pub natural_words: bool,
    pub daily_goal_minutes: u32,
}

#[tauri::command]
pub fn get_settings(
    state: State<'_, EngineState>,
) -> Result<SettingsDto, String> {
    let session = state.session.lock().map_err(|e| e.to_string())?;
    Ok(SettingsDto {
        target_wpm: session.target_wpm(),
        error_mode: match session.error_mode {
            ErrorMode::ForgiveMistakes => "forgive".to_string(),
            ErrorMode::StopOnError => "stop".to_string(),
        },
        fragment_length: session.fragment_length,
        natural_words: session.natural_words,
        daily_goal_minutes: session.daily_goal_minutes,
    })
}

#[tauri::command]
pub fn update_settings(
    state: State<'_, EngineState>,
    settings: SettingsDto,
) -> Result<(), String> {
    let mut session = state.session.lock().map_err(|e| e.to_string())?;
    session.set_target_wpm(settings.target_wpm);
    session.error_mode = match settings.error_mode.as_str() {
        "stop" => ErrorMode::StopOnError,
        _ => ErrorMode::ForgiveMistakes,
    };
    session.fragment_length = settings.fragment_length;
    session.natural_words = settings.natural_words;
    session.daily_goal_minutes = settings.daily_goal_minutes;
    // Persist settings as JSON alongside stats
    let saved = session.to_saved_stats();
    let _ = saved.save();
    Ok(())
}
