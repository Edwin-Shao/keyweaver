import { invoke } from "@tauri-apps/api/core";
import type {
  SessionState,
  ProcessResult,
  KeyboardState,
  FullStats,
  Settings,
} from "../types";

export async function startLesson(): Promise<SessionState> {
  return invoke("start_lesson");
}

export async function getSessionState(): Promise<SessionState> {
  return invoke("get_session_state");
}

export async function processKeystroke(typed: string): Promise<ProcessResult> {
  return invoke("process_keystroke", { typed });
}

export async function typingBackspace(): Promise<SessionState> {
  return invoke("typing_backspace");
}

export async function getKeyboardState(): Promise<KeyboardState> {
  return invoke("get_keyboard_state");
}

export async function getFullStats(): Promise<FullStats> {
  return invoke("get_full_stats");
}

export async function getSettings(): Promise<Settings> {
  return invoke("get_settings");
}

export async function updateSettings(settings: Settings): Promise<void> {
  return invoke("update_settings", { settings });
}
