import { useAppStore } from "../stores/appStore";

export default function TranslationDisplay() {
  const translation = useAppStore((s) => s.session?.currentTranslation);

  return (
    <div
      style={{
        textAlign: "center",
        fontSize: 28,
        fontWeight: 600,
        color: "#ffd93d",
        minHeight: 44,
        lineHeight: "44px",
        opacity: translation ? 1 : 0,
        transition: "opacity 0.1s",
        letterSpacing: 2,
      }}
    >
      {translation || " "}
    </div>
  );
}
