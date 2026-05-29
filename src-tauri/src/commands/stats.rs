use serde::Serialize;
use tauri::State;

use super::session::{EngineState, LessonResultDto};
use crate::state::lesson_score;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyDto {
    pub char: char,
    pub is_active: bool,
    pub confidence: f64,
    pub attempts: u32,
    pub errors: u32,
    pub wpm: Option<f64>,
    pub best_wpm: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyboardState {
    pub keys: Vec<KeyDto>,
    pub focused_key: Option<char>,
    pub active_keys: Vec<char>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullStats {
    pub per_key: std::collections::HashMap<char, KeyDto>,
    pub active_keys: Vec<char>,
    pub focused_key: Option<char>,
    pub total_lessons: u32,
    pub last_lesson: Option<LessonResultDto>,
    pub lesson_history: Vec<LessonResultDto>,
    pub today_seconds_practiced: u32,
    pub daily_goal_minutes: u32,
}

#[tauri::command]
pub fn get_keyboard_state(
    state: State<'_, EngineState>,
) -> Result<KeyboardState, String> {
    let session = state.session.lock().map_err(|e| e.to_string())?;
    let active_set: std::collections::HashSet<char> =
        session.scheduler.active_keys.iter().copied().collect();

    let keys: Vec<KeyDto> = ('a'..='z')
        .map(|ch| {
            let is_active = active_set.contains(&ch);
            let ks = session.per_key_stats.get(&ch);
            KeyDto {
                char: ch,
                is_active,
                confidence: ks.map(|k| k.best_confidence(session.target_cpm)).unwrap_or(0.0),
                attempts: ks.map(|k| k.attempts).unwrap_or(0),
                errors: ks.map(|k| k.errors).unwrap_or(0),
                wpm: ks.and_then(|k| k.wpm()),
                best_wpm: ks.and_then(|k| k.best_wpm()),
            }
        })
        .collect();

    Ok(KeyboardState {
        keys,
        focused_key: session.scheduler.focused_key,
        active_keys: session.scheduler.active_keys.clone(),
    })
}

#[tauri::command]
pub fn get_full_stats(
    state: State<'_, EngineState>,
) -> Result<FullStats, String> {
    let session = state.session.lock().map_err(|e| e.to_string())?;
    let mut per_key = std::collections::HashMap::new();

    for ch in 'a'..='z' {
        if let Some(ks) = session.per_key_stats.get(&ch) {
            per_key.insert(ch, KeyDto {
                char: ch,
                is_active: session.scheduler.active_keys.contains(&ch),
                confidence: ks.best_confidence(session.target_cpm),
                attempts: ks.attempts,
                errors: ks.errors,
                wpm: ks.wpm(),
                best_wpm: ks.best_wpm(),
            });
        }
    }

    Ok(FullStats {
        per_key,
        active_keys: session.scheduler.active_keys.clone(),
        focused_key: session.scheduler.focused_key,
        total_lessons: session.lesson_count,
        last_lesson: session.last_lesson.as_ref().map(|r| LessonResultDto {
            wpm: r.wpm,
            accuracy: r.accuracy,
            score: lesson_score(r.wpm, r.accuracy),
            newly_unlocked: r.newly_unlocked,
        }),
        lesson_history: session.lesson_history.iter().map(|r| LessonResultDto {
            wpm: r.wpm,
            accuracy: r.accuracy,
            score: lesson_score(r.wpm, r.accuracy),
            newly_unlocked: r.newly_unlocked,
        }).collect(),
        today_seconds_practiced: session.today_seconds_practiced,
        daily_goal_minutes: session.daily_goal_minutes,
    })
}
