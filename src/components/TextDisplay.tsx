import { useAppStore } from "../stores/appStore";

export default function TextDisplay() {
  const session = useAppStore((s) => s.session);
  if (!session) return null;

  const { text, cursorPos, errorPositions, recoveredPositions } = session;
  const chars: string[] = Array.from(text);
  const errorSet = new Set(errorPositions);
  const recoveredSet = new Set(recoveredPositions);

  return (
    <div
      style={{
        textAlign: "left",
        fontFamily: "'JetBrains Mono', 'Fira Code', 'Cascadia Code', 'SF Mono', monospace",
        fontSize: 22,
        lineHeight: 2.0,
        letterSpacing: 0.5,
        padding: "24px 32px",
        wordBreak: "break-word",
        overflowWrap: "break-word",
        whiteSpace: "normal",
        maxWidth: "100%",
        overflow: "hidden",
      }}
    >
      {chars.map((ch, i) => {
        let color = "#555";
        let bg = "transparent";
        let borderBottom = "none";

        if (i < cursorPos) {
          if (errorSet.has(i)) {
            color = "#ff6b6b";
            bg = "rgba(255,100,100,0.15)";
          } else if (recoveredSet.has(i)) {
            color = "#ffd93d";
          } else {
            color = "#6bdf6b";
          }
        } else if (i === cursorPos) {
          color = "#fff";
          borderBottom = "2px solid #6b8aff";
        }

        return (
          <span
            key={i}
            style={{
              color,
              background: bg,
              borderBottom,
              borderRadius: 3,
              display: "inline",
            }}
          >
            {ch === " " ? " " : ch}
          </span>
        );
      })}
    </div>
  );
}
