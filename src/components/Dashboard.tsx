import { useAppStore } from "../stores/appStore";

export default function Dashboard() {
  const session = useAppStore((s) => s.session);
  const lastResult = useAppStore((s) => s.lastResult);

  if (!session) return null;

  return (
    <div
      style={{
        display: "flex",
        justifyContent: "center",
        gap: 48,
        fontSize: 15,
        color: "#999",
        paddingBottom: 8,
        fontFamily: "monospace",
      }}
    >
      <span>
        <span style={{ color: "#6bdf6b", fontSize: 22, fontWeight: 700 }}>
          {session.wpm.toFixed(0)}
        </span>
        {" wpm"}
      </span>

      <span>
        <span style={{ color: "#ffd93d", fontSize: 22, fontWeight: 700 }}>
          {session.accuracy.toFixed(1)}%
        </span>
        {" acc"}
      </span>

      {lastResult && (
        <span style={{ color: "#6bd4ff", fontSize: 14 }}>
          Last: {lastResult.wpm.toFixed(0)} wpm / {lastResult.accuracy.toFixed(0)}% / {lastResult.score.toFixed(0)} pts
        </span>
      )}
    </div>
  );
}
