import { useAppStore } from "../stores/appStore";

export default function TextDisplay() {
  const session = useAppStore((s) => s.session);
  if (!session) return null;

  const { text, cursorPos, errorPositions, recoveredPositions } = session;
  const errorSet = new Set(errorPositions);
  const recoveredSet = new Set(recoveredPositions);

  // Split text into words (keeping spaces), then render each word
  // wrapped in a nowrap container so it never breaks mid-word
  const words: string[] = [];
  let current = "";
  for (const ch of text) {
    if (ch === " ") {
      if (current) words.push(current);
      words.push(" ");
      current = "";
    } else {
      current += ch;
    }
  }
  if (current) words.push(current);

  let globalIndex = 0;

  return (
    <div
      style={{
        display: "flex",
        flexWrap: "wrap",
        fontFamily: "'JetBrains Mono', 'Fira Code', 'Cascadia Code', 'SF Mono', monospace",
        fontSize: 22,
        lineHeight: 2.0,
        letterSpacing: 0.5,
        padding: "24px 32px",
        width: "100%",
      }}
    >
      {words.map((word, wi) => {
        const isSpace = word === " ";
        const chars = Array.from(word);
        const startIdx = globalIndex;
        globalIndex += chars.length;

        return (
          <span
            key={wi}
            style={{
              whiteSpace: isSpace ? "normal" : "nowrap",
              display: "inline",
            }}
          >
            {chars.map((ch, ci) => {
              const i = startIdx + ci;
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
                  key={ci}
                  style={{ color, background: bg, borderBottom, borderRadius: 3 }}
                >
                  {ch === " " ? " " : ch}
                </span>
              );
            })}
          </span>
        );
      })}
    </div>
  );
}
