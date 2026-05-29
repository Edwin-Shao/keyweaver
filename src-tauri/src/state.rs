use std::collections::{HashMap, HashSet};
use std::time::Instant;

use serde::Serialize;

use crate::engine::{LetterFilter, LetterScheduler, Translations, WordGenerator};
use crate::engine::metrics::KeyStats;
use crate::persistence::{SavedLessonResult, SavedStats};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorMode {
    ForgiveMistakes,
    StopOnError,
}

#[derive(Debug, Clone, Serialize)]
pub struct LessonResult {
    pub wpm: f64,
    pub accuracy: f64,
    pub score: f64,
    pub newly_unlocked: Option<char>,
}

pub fn lesson_score(wpm: f64, accuracy: f64) -> f64 {
    let acc_ratio = (accuracy / 100.0).clamp(0.0, 1.0);
    (wpm * acc_ratio * acc_ratio * 100.0).round()
}

pub struct AppSession {
    // Text state
    pub generated_text: String,
    pub cursor_pos: usize,
    pub error_positions: HashSet<usize>,
    pub first_attempt_correct: HashSet<usize>,
    pub recovered_positions: HashSet<usize>,
    pub ever_error_positions: HashSet<usize>,

    // Per-lesson metrics
    pub lesson_start: Option<Instant>,
    pub key_target_start: Option<Instant>,
    pub lesson_correct: u32,
    pub lesson_positions: u32,
    pub lesson_errors: u32,
    pub lesson_count: u32,

    // Cumulative per-key stats
    pub per_key_stats: HashMap<char, KeyStats>,

    // Last lesson result + history
    pub last_lesson: Option<LessonResult>,
    pub lesson_history: Vec<LessonResult>,

    // Engine components
    pub scheduler: LetterScheduler,
    pub generator: WordGenerator,
    pub translations: Translations,

    // Settings
    pub target_cpm: f64,
    pub error_mode: ErrorMode,
    pub fragment_length: usize,
    pub natural_words: bool,
    pub daily_goal_minutes: u32,
    pub today_seconds_practiced: u32,
    pub today_date: String,

    // Translation display
    pub current_translation: Option<String>,
}

impl AppSession {
    pub fn new() -> Self {
        Self::new_with_state(None)
    }

    pub fn new_with_state(saved: Option<SavedStats>) -> Self {
        let mut scheduler = LetterScheduler::new();
        let mut stats: HashMap<char, KeyStats> = HashMap::new();
        let mut lesson_count: u32 = 0;
        let today = crate::persistence::today_date_string();
        let mut today_seconds_practiced: u32 = 0;
        let mut today_date: String = today.clone();
        let mut last_lesson: Option<LessonResult> = None;
        let mut lesson_history: Vec<LessonResult> = Vec::new();

        if let Some(saved) = saved {
            for (ch, sk) in &saved.keys {
                let ks = stats.entry(*ch).or_default();
                ks.attempts = sk.attempts;
                ks.errors = sk.errors;
                ks.filtered_time_ms = sk.filtered_time_ms;
                ks.best_filtered_time_ms = sk.best_filtered_time_ms;
                let recent = &sk.recent_times_ms;
                let start = recent.len().saturating_sub(20);
                ks.reaction_times_ms = recent[start..].to_vec();
            }

            if saved.unlocked_letters.len() >= 6 {
                scheduler.active_keys = saved.unlocked_letters;
                scheduler.set_unlock_index_from_active();
            }

            lesson_count = saved.total_lessons;

            if saved.today_date == today && !today.is_empty() {
                today_seconds_practiced = saved.today_seconds_practiced;
                today_date = saved.today_date;
            }

            last_lesson = saved.last_lesson.map(|r| LessonResult {
                wpm: r.wpm,
                accuracy: r.accuracy,
                score: lesson_score(r.wpm, r.accuracy),
                newly_unlocked: None,
            });
            lesson_history = saved
                .lesson_history
                .into_iter()
                .map(|r| LessonResult {
                    wpm: r.wpm,
                    accuracy: r.accuracy,
                    score: lesson_score(r.wpm, r.accuracy),
                    newly_unlocked: None,
                })
                .collect();
        }

        scheduler.update(&stats, 175.0); // default 35 WPM target
        let filter = LetterFilter::new(&scheduler.active_keys, scheduler.focused_key);
        let mut generator = WordGenerator::new();
        generator.set_natural_words(true);
        let text = generator.generate_fragment(&filter, 100);
        let translations = Translations::from_embedded();

        let mut session = Self {
            generated_text: text,
            cursor_pos: 0,
            error_positions: HashSet::new(),
            first_attempt_correct: HashSet::new(),
            recovered_positions: HashSet::new(),
            ever_error_positions: HashSet::new(),
            lesson_start: None,
            key_target_start: None,
            lesson_correct: 0,
            lesson_positions: 0,
            lesson_errors: 0,
            lesson_count,
            per_key_stats: stats,
            last_lesson,
            lesson_history,
            scheduler,
            generator,
            translations,
            target_cpm: 175.0,
            error_mode: ErrorMode::ForgiveMistakes,
            fragment_length: 100,
            natural_words: true,
            daily_goal_minutes: 30,
            today_seconds_practiced,
            today_date,
            current_translation: None,
        };
        session.update_current_translation();
        session
    }

    pub fn target_wpm(&self) -> u32 {
        (self.target_cpm / 5.0).round() as u32
    }

    pub fn set_target_wpm(&mut self, wpm: u32) {
        self.target_cpm = wpm as f64 * 5.0;
    }

    pub fn lesson_wpm(&self) -> f64 {
        let start = match self.lesson_start {
            Some(s) => s,
            None => return 0.0,
        };
        let elapsed = start.elapsed().as_secs_f64();
        if elapsed < 1.0 { return 0.0; }
        (self.first_attempt_correct.len() as f64 / 5.0) / (elapsed / 60.0)
    }

    pub fn lesson_accuracy(&self) -> f64 {
        if self.lesson_positions == 0 { return 100.0; }
        ((self.lesson_positions - self.lesson_errors) as f64 / self.lesson_positions as f64) * 100.0
    }

    pub fn process_char(&mut self, typed: char) -> ProcessCharResult {
        let target = match self.generated_text.chars().nth(self.cursor_pos) {
            Some(c) => c,
            None => return ProcessCharResult::default(),
        };

        if self.lesson_start.is_none() {
            self.lesson_start = Some(Instant::now());
            self.key_target_start = Some(Instant::now());
        }

        let reaction_ms = self.key_target_start
            .map(|t| t.elapsed().as_millis() as u64)
            .unwrap_or(0);

        if typed == target {
            let pos = self.cursor_pos;
            if self.ever_error_positions.contains(&pos) {
                self.recovered_positions.insert(pos);
            } else {
                self.first_attempt_correct.insert(pos);
            }

            if target != ' ' && self.first_attempt_correct.contains(&pos) {
                let stats = self.per_key_stats.entry(target).or_default();
                if (40..=12000).contains(&reaction_ms) {
                    stats.record_hit(reaction_ms);
                }
                self.lesson_positions += 1;
            }
            self.lesson_correct += 1;
            self.cursor_pos += 1;
            self.key_target_start = Some(Instant::now());
            self.update_current_translation();

            if self.cursor_pos >= self.generated_text.chars().count() {
                return self.finish_lesson_result();
            }

            ProcessCharResult {
                lesson_complete: false,
                ..Default::default()
            }
        } else {
            let pos = self.cursor_pos;
            if target != ' ' {
                let stats = self.per_key_stats.entry(target).or_default();
                stats.record_error();
                if !self.error_positions.contains(&pos) {
                    self.lesson_positions += 1;
                    self.lesson_errors += 1;
                }
            }
            self.ever_error_positions.insert(pos);

            match self.error_mode {
                ErrorMode::ForgiveMistakes => {
                    self.error_positions.insert(pos);
                    self.cursor_pos += 1;
                    self.key_target_start = Some(Instant::now());
                    self.update_current_translation();

                    if self.cursor_pos >= self.generated_text.chars().count() {
                        return self.finish_lesson_result();
                    }
                    ProcessCharResult { lesson_complete: false, ..Default::default() }
                }
                ErrorMode::StopOnError => {
                    self.error_positions.insert(pos);
                    self.key_target_start = Some(Instant::now());
                    ProcessCharResult { lesson_complete: false, ..Default::default() }
                }
            }
        }
    }

    fn finish_lesson_result(&mut self) -> ProcessCharResult {
        self.lesson_count += 1;
        let wpm = self.lesson_wpm();
        let accuracy = self.lesson_accuracy();
        let old_count = self.scheduler.active_keys.len();

        self.scheduler.update(&self.per_key_stats, self.target_cpm);

        let newly_unlocked = if self.scheduler.active_keys.len() > old_count {
            Some(*self.scheduler.active_keys.last().unwrap())
        } else {
            None
        };

        let result = LessonResult {
            wpm, accuracy,
            score: lesson_score(wpm, accuracy),
            newly_unlocked,
        };

        const HISTORY_CAP: usize = 50;
        self.lesson_history.push(result.clone());
        if self.lesson_history.len() > HISTORY_CAP {
            let overflow = self.lesson_history.len() - HISTORY_CAP;
            self.lesson_history.drain(0..overflow);
        }
        self.last_lesson = Some(result.clone());

        // Start next lesson
        self.generator.set_natural_words(self.natural_words);
        let filter = LetterFilter::new(&self.scheduler.active_keys, self.scheduler.focused_key);
        self.generated_text = self.generator.generate_fragment(&filter, self.fragment_length);
        self.cursor_pos = 0;
        self.error_positions.clear();
        self.first_attempt_correct.clear();
        self.recovered_positions.clear();
        self.ever_error_positions.clear();
        self.key_target_start = None;
        self.lesson_start = None;
        self.lesson_correct = 0;
        self.lesson_positions = 0;
        self.lesson_errors = 0;
        self.update_current_translation();

        ProcessCharResult {
            lesson_complete: true,
            lesson_result: Some(result),
            new_key_unlocked: newly_unlocked,
            ..Default::default()
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_pos == 0 { return; }
        self.cursor_pos -= 1;
        self.error_positions.remove(&self.cursor_pos);
        self.first_attempt_correct.remove(&self.cursor_pos);
        self.recovered_positions.remove(&self.cursor_pos);
        self.key_target_start = Some(Instant::now());
        self.update_current_translation();
    }

    pub fn update_current_translation(&mut self) {
        self.current_translation = self.word_around_cursor()
            .and_then(|w| self.translations.get(w).map(|s| s.to_string()));
    }

    fn word_around_cursor(&self) -> Option<&str> {
        let text = self.generated_text.as_str();
        if text.is_empty() { return None; }
        let len = text.len();
        let raw_pos = self.cursor_pos.min(len.saturating_sub(1));

        let pos = if text.as_bytes().get(raw_pos) == Some(&b' ') {
            let next = raw_pos + 1;
            if next < len && text.as_bytes().get(next) != Some(&b' ') { next }
            else if raw_pos > 0 { raw_pos - 1 }
            else { return None; }
        } else {
            raw_pos
        };

        let start = text[..=pos].rfind(' ').map(|i| i + 1).unwrap_or(0);
        let end = text[pos..].find(' ').map(|i| pos + i).unwrap_or(len);
        if start >= end { return None; }
        let word = &text[start..end];
        if word.is_empty() { None } else { Some(word) }
    }

    pub fn to_saved_stats(&self) -> SavedStats {
        let mut keys = HashMap::new();
        for (ch, ks) in &self.per_key_stats {
            keys.insert(*ch, crate::persistence::SavedKeyStats {
                attempts: ks.attempts,
                errors: ks.errors,
                filtered_time_ms: ks.filtered_time_ms,
                best_filtered_time_ms: ks.best_filtered_time_ms,
                recent_times_ms: ks.reaction_times_ms.clone(),
            });
        }
        SavedStats {
            version: 2,
            keys,
            unlocked_letters: self.scheduler.active_keys.clone(),
            total_lessons: self.lesson_count,
            last_session: format!("{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()),
            today_seconds_practiced: self.today_seconds_practiced,
            today_minutes_practiced: None,
            today_date: self.today_date.clone(),
            last_lesson: self.last_lesson.as_ref().map(|r| SavedLessonResult {
                wpm: r.wpm, accuracy: r.accuracy,
            }),
            lesson_history: self.lesson_history.iter().map(|r| SavedLessonResult {
                wpm: r.wpm, accuracy: r.accuracy,
            }).collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessCharResult {
    pub lesson_complete: bool,
    pub lesson_result: Option<LessonResult>,
    pub new_key_unlocked: Option<char>,
}
