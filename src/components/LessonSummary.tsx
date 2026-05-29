import type { LessonResult } from "../types";

interface Props {
  result: LessonResult;
  newKey: string | null;
}

export default function LessonSummary({ result, newKey }: Props) {
  return (
    <div
      style={{
        position: "absolute",
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        background: "rgba(0,0,0,0.6)",
        zIndex: 100,
        borderRadius: 12,
        gap: 12,
      }}
    >
      <h2 style={{ fontSize: 32, color: "#fff", fontWeight: 700 }}>Lesson Complete</h2>

      <div style={{ display: "flex", gap: 32, fontSize: 22, color: "#ccc" }}>
        <span>{result.wpm.toFixed(0)} <span style={{ color: "#6bdf6b" }}>wpm</span></span>
        <span>{result.accuracy.toFixed(1)}<span style={{ color: "#ffd93d" }}>%</span></span>
        <span>{result.score.toFixed(0)} <span style={{ color: "#6bd4ff" }}>pts</span></span>
      </div>

      {newKey && (
        <div style={{ fontSize: 20, color: "#4ade80", marginTop: 8 }}>
          + &apos;{newKey}&apos; unlocked!
        </div>
      )}

      <div style={{ fontSize: 14, color: "#666", marginTop: 16 }}>
        continue typing...
      </div>
    </div>
  );
}
