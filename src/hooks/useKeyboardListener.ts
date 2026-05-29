import { useEffect, useCallback } from "react";
import { useAppStore } from "../stores/appStore";
import {
  processKeystroke,
  typingBackspace,
  getKeyboardState,
  getSessionState,
} from "../lib/commands";
import type { ProcessResult } from "../types";

export function useKeyboardListener() {
  const { setSession, setKeyboardState, setScreen } = useAppStore();

  useEffect(() => {
    // Load initial state and keyboard
    getSessionState().then((s) => {
      if (s.text) setSession(s);
    });
    getKeyboardState().then(setKeyboardState);
  }, []);

  const handleKey = useCallback(
    async (e: KeyboardEvent) => {
      // Ignore modifier-only keys and IME composition
      if (e.isComposing) return;
      if (["Shift", "Control", "Alt", "Meta", "CapsLock", "Tab"].includes(e.key))
        return;

      e.preventDefault();

      if (e.key === "Escape") {
        setScreen("menu");
        return;
      }

      if (e.key === "Backspace") {
        const state = await typingBackspace();
        setSession(state);
        return;
      }

      // Map Enter to space (lesson text doesn't contain newlines)
      const char = e.key === "Enter" ? " " : e.key;

      // Only process printable single characters + space
      if (char.length === 1) {
        const result: ProcessResult = await processKeystroke(char);
        setSession(result.session);

        if (result.lessonComplete) {
          useAppStore.setState({
            lastResult: result.lessonResult,
            newKeyUnlocked: result.newKeyUnlocked,
          });
          // After a brief pause, continue to next lesson
          setTimeout(() => {
            const s = useAppStore.getState().session;
            if (s) {
              useAppStore.setState({ lastResult: null, newKeyUnlocked: null });
            }
          }, 2500);
        }

        // Refresh keyboard state periodically
        getKeyboardState().then(setKeyboardState);
      }
    },
    [setSession, setKeyboardState, setScreen],
  );

  useEffect(() => {
    window.addEventListener("keydown", handleKey);
    return () => window.removeEventListener("keydown", handleKey);
  }, [handleKey]);
}
