use serde::Serialize;
use tauri::State;

use super::session::{EngineState, SessionState, session_to_state};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessResult {
    pub session: SessionState,
    pub lesson_complete: bool,
    pub lesson_result: Option<super::session::LessonResultDto>,
    pub new_key_unlocked: Option<char>,
}

#[tauri::command]
pub fn process_keystroke(
    state: State<'_, EngineState>,
    typed: String,
) -> Result<ProcessResult, String> {
    let mut session = state.session.lock().map_err(|e| e.to_string())?;
    let c = typed.chars().next().unwrap_or(' ');

    let result = session.process_char(c);

    let lesson_result = result.lesson_result.as_ref().map(|lr| {
        super::session::LessonResultDto {
            wpm: lr.wpm,
            accuracy: lr.accuracy,
            score: lr.score,
            newly_unlocked: lr.newly_unlocked,
        }
    });
    let new_key_unlocked = result.new_key_unlocked;
    let lesson_complete = result.lesson_complete;

    let session_state = session_to_state(&session);

    if result.lesson_complete {
        let saved = session.to_saved_stats();
        let _ = saved.save();
    }

    Ok(ProcessResult {
        session: session_state,
        lesson_complete,
        lesson_result,
        new_key_unlocked,
    })
}

#[tauri::command]
pub fn typing_backspace(
    state: State<'_, EngineState>,
) -> Result<SessionState, String> {
    let mut session = state.session.lock().map_err(|e| e.to_string())?;
    session.backspace();
    Ok(session_to_state(&session))
}
