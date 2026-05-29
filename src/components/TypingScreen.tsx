import { useKeyboardListener } from "../hooks/useKeyboardListener";
import { useAppStore } from "../stores/appStore";
import TextDisplay from "./TextDisplay";
import TranslationDisplay from "./TranslationDisplay";
import Dashboard from "./Dashboard";
import KeyHeatmap from "./KeyHeatmap";
import LessonSummary from "./LessonSummary";
import Footer from "./Footer";

export default function TypingScreen() {
  useKeyboardListener();
  const { session, lastResult, newKeyUnlocked } = useAppStore();

  if (!session) {
    return (
      <div style={{ height: "100%", display: "flex", alignItems: "center", justifyContent: "center", color: "#666" }}>
        Loading...
      </div>
    );
  }

  return (
    <div
      style={{
        height: "100%",
        display: "flex",
        flexDirection: "column",
        padding: "32px 48px",
        userSelect: "none",
        WebkitUserSelect: "none",
      }}
    >
      {lastResult && <LessonSummary result={lastResult} newKey={newKeyUnlocked} />}

      <Dashboard />
      <div style={{ flex: 1, display: "flex", flexDirection: "column", justifyContent: "center" }}>
        <TextDisplay />
        <TranslationDisplay />
      </div>
      <KeyHeatmap />
      <Footer />
    </div>
  );
}
