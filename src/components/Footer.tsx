import { useAppStore } from "../stores/appStore";

export default function Footer() {
  const session = useAppStore((s) => s.session);
  if (!session) return null;

  return (
    <div
      style={{
        textAlign: "center",
        fontSize: 12,
        color: "#444",
        fontFamily: "monospace",
        paddingTop: 8,
      }}
    >
      Lesson {session.lessonCount + 1} · Esc to menu · Backspace to correct · Tab to switch mode
    </div>
  );
}
