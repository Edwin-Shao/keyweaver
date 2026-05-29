use serde::Serialize;
use tauri::State;

use crate::persistence::SavedStats;
use crate::state::AppSession;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionState {
    pub text: String,
    pub cursor_pos: usize,
    pub error_positions: Vec<usize>,
    pub first_attempt_correct: Vec<usize>,
    pub recovered_positions: Vec<usize>,
    pub wpm: f64,
    pub accuracy: f64,
    pub lesson_count: u32,
    pub current_translation: Option<String>,
    pub active_keys: Vec<char>,
    pub focused_key: Option<char>,
    pub lesson_complete: bool,
    pub lesson_result: Option<LessonResultDto>,
    pub new_key_unlocked: Option<char>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonResultDto {
    pub wpm: f64,
    pub accuracy: f64,
    pub score: f64,
    pub newly_unlocked: Option<char>,
}

pub fn session_to_state(session: &AppSession) -> SessionState {
    SessionState {
        text: session.generated_text.clone(),
        cursor_pos: session.cursor_pos,
        error_positions: session.error_positions.iter().copied().collect(),
        first_attempt_correct: session.first_attempt_correct.iter().copied().collect(),
        recovered_positions: session.recovered_positions.iter().copied().collect(),
        wpm: session.lesson_wpm(),
        accuracy: session.lesson_accuracy(),
        lesson_count: session.lesson_count,
        current_translation: session.current_translation.clone(),
        active_keys: session.scheduler.active_keys.clone(),
        focused_key: session.scheduler.focused_key,
        lesson_complete: false,
        lesson_result: None,
        new_key_unlocked: None,
    }
}

pub struct EngineState {
    pub session: std::sync::Mutex<AppSession>,
}

impl EngineState {
    pub fn new() -> Self {
        let saved = SavedStats::load();
        let session = AppSession::new_with_state(saved);
        Self {
            session: std::sync::Mutex::new(session),
        }
    }
}

#[tauri::command]
pub fn start_lesson(
    state: State<'_, EngineState>,
) -> Result<SessionState, String> {
    let mut session = state.session.lock().map_err(|e| e.to_string())?;
    let natural_words = session.natural_words;
    let fragment_length = session.fragment_length;
    let filter = crate::engine::LetterFilter::new(
        &session.scheduler.active_keys,
        session.scheduler.focused_key,
    );
    session.generator.set_natural_words(natural_words);
    session.generated_text = session.generator.generate_fragment(&filter, fragment_length);
    session.cursor_pos = 0;
    session.error_positions.clear();
    session.first_attempt_correct.clear();
    session.recovered_positions.clear();
    session.ever_error_positions.clear();
    session.lesson_start = None;
    session.lesson_correct = 0;
    session.lesson_positions = 0;
    session.lesson_errors = 0;
    session.update_current_translation();
    Ok(session_to_state(&session))
}

#[tauri::command]
pub fn get_session_state(
    state: State<'_, EngineState>,
) -> Result<SessionState, String> {
    let session = state.session.lock().map_err(|e| e.to_string())?;
    Ok(session_to_state(&session))
}
