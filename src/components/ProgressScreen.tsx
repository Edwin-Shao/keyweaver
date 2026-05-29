import { useEffect } from "react";
import { useAppStore } from "../stores/appStore";

export default function ProgressScreen() {
  const { fullStats, loadFullStats, setScreen } = useAppStore();

  useEffect(() => {
    loadFullStats();
  }, []);

  if (!fullStats) {
    return (
      <div style={{ height: "100%", display: "flex", alignItems: "center", justifyContent: "center", color: "#666" }}>
        Loading...
      </div>
    );
  }

  const { totalLessons, lessonHistory, todaySecondsPracticed, dailyGoalMinutes, lastLesson } = fullStats;
  const dailyMin = todaySecondsPracticed / 60;
  const goalPercent = dailyGoalMinutes > 0 ? Math.min(100, (dailyMin / dailyGoalMinutes) * 100) : 0;

  return (
    <div style={{ height: "100%", padding: "40px 48px", overflow: "auto" }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: 32 }}>
        <h2 style={{ fontSize: 28, fontWeight: 700, color: "#fff" }}>Progress</h2>
        <button
          onClick={() => setScreen("menu")}
          style={{
            padding: "8px 20px", fontSize: 14, color: "#999", background: "#2a2a3a", borderRadius: 8,
          }}
        >
          Back
        </button>
      </div>

      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 24, marginBottom: 32 }}>
        <StatBox label="Total Lessons" value={String(totalLessons)} />
        <StatBox label="Last WPM" value={lastLesson ? lastLesson.wpm.toFixed(0) : "—"} />
        <StatBox label="Last Accuracy" value={lastLesson ? `${lastLesson.accuracy.toFixed(1)}%` : "—"} />
        <StatBox label="Last Score" value={lastLesson ? lastLesson.score.toFixed(0) : "—"} />
      </div>

      <div style={{ marginBottom: 24 }}>
        <div style={{ fontSize: 14, color: "#888", marginBottom: 6 }}>
          Daily Goal: {dailyMin.toFixed(0)} / {dailyGoalMinutes} min
        </div>
        <div style={{ height: 8, background: "#2a2a3a", borderRadius: 4 }}>
          <div style={{ height: "100%", width: `${goalPercent}%`, background: "#6bdf6b", borderRadius: 4, transition: "width 0.5s" }} />
        </div>
      </div>

      <h3 style={{ fontSize: 16, color: "#999", marginBottom: 12 }}>Lesson History</h3>
      {lessonHistory.length === 0 ? (
        <div style={{ color: "#555", fontSize: 14 }}>No lessons completed yet.</div>
      ) : (
        <div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
          {lessonHistory.slice(-20).reverse().map((r, i) => (
            <div key={i} style={{ display: "flex", gap: 24, fontSize: 14, color: "#777", fontFamily: "monospace" }}>
              <span>#{lessonHistory.length - i}</span>
              <span style={{ color: "#6bdf6b" }}>{r.wpm.toFixed(0)} wpm</span>
              <span style={{ color: "#ffd93d" }}>{r.accuracy.toFixed(1)}%</span>
              <span>{r.score.toFixed(0)} pts</span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

function StatBox({ label, value }: { label: string; value: string }) {
  return (
    <div style={{ background: "#1a1a2a", borderRadius: 10, padding: "16px 20px" }}>
      <div style={{ fontSize: 12, color: "#666", marginBottom: 4 }}>{label}</div>
      <div style={{ fontSize: 24, fontWeight: 700, color: "#fff" }}>{value}</div>
    </div>
  );
}
