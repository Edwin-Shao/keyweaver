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
        textAlign: "center",
        fontFamily: "'JetBrains Mono', 'Fira Code', 'Cascadia Code', 'SF Mono', monospace",
        fontSize: 26,
        lineHeight: 1.8,
        letterSpacing: 1,
        wordSpacing: 6,
        padding: "16px 0",
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
              borderRadius: bg !== "transparent" ? 3 : 0,
              padding: bg !== "transparent" ? "0 2px" : 0,
            }}
          >
            {ch === " " ? " " : ch}
          </span>
        );
      })}
    </div>
  );
}
