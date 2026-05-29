use std::collections::HashMap;
use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

fn default_version() -> u32 { 1 }
fn default_today_seconds_practiced() -> u32 { 0 }
fn default_today_date() -> String { String::new() }
fn default_legacy_minutes() -> Option<u32> { None }
fn default_last_lesson() -> Option<SavedLessonResult> { None }
fn default_lesson_history() -> Vec<SavedLessonResult> { Vec::new() }

pub fn today_date_string() -> String {
    use std::time::SystemTime;
    let secs = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => d.as_secs() as i64,
        Err(_) => return String::new(),
    };
    let z: i64 = secs.div_euclid(86_400) + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe: u32 = (z - era * 146_097) as u32;
    let yoe: u32 = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y: i64 = yoe as i64 + era * 400;
    let doy: u32 = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp: u32 = (5 * doy + 2) / 153;
    let d: u32 = doy - (153 * mp + 2) / 5 + 1;
    let m: u32 = if mp < 10 { mp + 3 } else { mp - 9 };
    let year: i64 = if m <= 2 { y + 1 } else { y };
    format!("{:04}-{:02}-{:02}", year, m, d)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedKeyStats {
    pub attempts: u32,
    pub errors: u32,
    pub filtered_time_ms: f64,
    pub best_filtered_time_ms: f64,
    pub recent_times_ms: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedLessonResult {
    pub wpm: f64,
    pub accuracy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedStats {
    #[serde(default = "default_version")]
    pub version: u32,
    pub keys: HashMap<char, SavedKeyStats>,
    pub unlocked_letters: Vec<char>,
    pub total_lessons: u32,
    pub last_session: String,
    #[serde(default = "default_today_seconds_practiced")]
    pub today_seconds_practiced: u32,
    #[serde(default = "default_legacy_minutes", skip_serializing)]
    pub today_minutes_practiced: Option<u32>,
    #[serde(default = "default_today_date")]
    pub today_date: String,
    #[serde(default = "default_last_lesson")]
    pub last_lesson: Option<SavedLessonResult>,
    #[serde(default = "default_lesson_history")]
    pub lesson_history: Vec<SavedLessonResult>,
}

impl SavedStats {
    pub fn path() -> Option<PathBuf> {
        ProjectDirs::from("", "", "KeyWeaver").map(|dirs| dirs.data_dir().join("stats.json"))
    }

    pub fn load() -> Option<Self> {
        let path = Self::path()?;
        if !path.exists() { return None; }
        let contents = std::fs::read_to_string(&path).ok()?;
        let mut stats: SavedStats = serde_json::from_str(&contents).ok()?;
        let today = today_date_string();
        if stats.today_date != today {
            stats.today_seconds_practiced = 0;
            stats.today_date = today;
        }
        if let Some(legacy_min) = stats.today_minutes_practiced.take() {
            if stats.today_seconds_practiced == 0 && legacy_min > 0 {
                stats.today_seconds_practiced = legacy_min.saturating_mul(60);
            }
        }
        Some(stats)
    }

    pub fn save(&self) -> Result<(), String> {
        let path = match Self::path() {
            Some(p) => p,
            None => return Ok(()),
        };
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        // Backup before overwriting
        if path.exists() {
            let backup = path.with_extension("json.bak");
            let _ = std::fs::copy(&path, &backup);
        }
        let contents = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(&path, contents).map_err(|e| e.to_string())
    }
}
