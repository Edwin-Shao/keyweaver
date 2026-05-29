import { create } from "zustand";
import type {
  AppScreen,
  SessionState,
  KeyboardState,
  FullStats,
  Settings,
  LessonResult,
} from "../types";
import {
  startLesson as startLessonCmd,
  getKeyboardState as getKeyboardStateCmd,
  getFullStats as getFullStatsCmd,
  getSettings as getSettingsCmd,
  updateSettings as updateSettingsCmd,
} from "../lib/commands";

interface AppStore {
  screen: AppScreen;
  setScreen: (s: AppScreen) => void;

  session: SessionState | null;
  setSession: (s: SessionState) => void;

  keyboardState: KeyboardState | null;
  setKeyboardState: (k: KeyboardState) => void;

  fullStats: FullStats | null;
  settings: Settings | null;

  lastResult: LessonResult | null;
  newKeyUnlocked: string | null;

  startLesson: () => Promise<void>;
  loadKeyboardState: () => Promise<void>;
  loadFullStats: () => Promise<void>;
  loadSettings: () => Promise<void>;
  saveSettings: (s: Settings) => Promise<void>;
}

export const useAppStore = create<AppStore>((set) => ({
  screen: "menu",
  setScreen: (screen) => set({ screen }),

  session: null,
  setSession: (session) => set({ session }),

  keyboardState: null,
  setKeyboardState: (keyboardState) => set({ keyboardState }),

  fullStats: null,
  settings: null,

  lastResult: null,
  newKeyUnlocked: null,

  startLesson: async () => {
    const session = await startLessonCmd();
    set({ session, screen: "typing", lastResult: null, newKeyUnlocked: null });
  },

  loadKeyboardState: async () => {
    const k = await getKeyboardStateCmd();
    set({ keyboardState: k });
  },

  loadFullStats: async () => {
    const s = await getFullStatsCmd();
    set({ fullStats: s });
  },

  loadSettings: async () => {
    const s = await getSettingsCmd();
    set({ settings: s });
  },

  saveSettings: async (settings) => {
    await updateSettingsCmd(settings);
    set({ settings });
  },
}));
