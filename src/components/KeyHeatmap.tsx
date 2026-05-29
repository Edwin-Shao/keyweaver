import { useAppStore } from "../stores/appStore";

function confidenceColor(conf: number, isActive: boolean): string {
  if (!isActive) return "#2a2a35";
  if (conf >= 1.0) return "#4ade80";
  if (conf >= 0.75) return "#a3e635";
  if (conf >= 0.5) return "#facc15";
  if (conf >= 0.25) return "#fb923c";
  return "#f87171";
}

export default function KeyHeatmap() {
  const keyboardState = useAppStore((s) => s.keyboardState);
  if (!keyboardState) return null;

  const { keys, focusedKey } = keyboardState;
  const rows = [
    keys.slice(0, 10),
    keys.slice(10, 19),
    keys.slice(19, 26),
  ];

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        gap: 4,
        padding: "12px 0",
      }}
    >
      {rows.map((row, ri) => (
        <div key={ri} style={{ display: "flex", gap: 4, marginLeft: ri === 1 ? 12 : 0 }}>
          {row.map((k) => (
            <div
              key={k.char}
              title={`${k.char}: ${(k.confidence * 100).toFixed(0)}%`}
              style={{
                width: 34,
                height: 34,
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
                fontSize: 13,
                fontWeight: 600,
                fontFamily: "monospace",
                color: k.isActive ? "#fff" : "#444",
                background: confidenceColor(k.confidence, k.isActive),
                borderRadius: 5,
                border: focusedKey === k.char ? "2px solid #fff" : "2px solid transparent",
                transition: "background 0.3s, border 0.3s",
              }}
            >
              {k.char.toUpperCase()}
            </div>
          ))}
        </div>
      ))}
    </div>
  );
}
