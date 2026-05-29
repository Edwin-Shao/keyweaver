export interface SessionState {
  text: string;
  cursorPos: number;
  errorPositions: number[];
  firstAttemptCorrect: number[];
  recoveredPositions: number[];
  wpm: number;
  accuracy: number;
  lessonCount: number;
  currentTranslation: string | null;
  activeKeys: string[];
  focusedKey: string | null;
}

export interface KeyDto {
  char: string;
  isActive: boolean;
  confidence: number;
  attempts: number;
  errors: number;
  wpm: number | null;
  bestWpm: number | null;
}

export interface KeyboardState {
  keys: KeyDto[];
  focusedKey: string | null;
  activeKeys: string[];
}

export interface LessonResult {
  wpm: number;
  accuracy: number;
  score: number;
  newlyUnlocked: string | null;
}

export interface FullStats {
  perKey: Record<string, KeyDto>;
  activeKeys: string[];
  focusedKey: string | null;
  totalLessons: number;
  lastLesson: LessonResult | null;
  lessonHistory: LessonResult[];
  todaySecondsPracticed: number;
  dailyGoalMinutes: number;
}

export interface ProcessResult {
  session: SessionState;
  lessonComplete: boolean;
  lessonResult: LessonResult | null;
  newKeyUnlocked: string | null;
}

export interface Settings {
  targetWpm: number;
  errorMode: string;
  fragmentLength: number;
  naturalWords: boolean;
  dailyGoalMinutes: number;
}

export type AppScreen = "menu" | "typing" | "progress" | "settings";
