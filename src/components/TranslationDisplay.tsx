import { useAppStore } from "../stores/appStore";

export default function TranslationDisplay() {
  const translation = useAppStore((s) => s.session?.currentTranslation);

  return (
    <div
      style={{
        textAlign: "center",
        fontSize: 18,
        color: "#6bd4ff",
        minHeight: 32,
        lineHeight: "32px",
        opacity: translation ? 0.85 : 0,
        transition: "opacity 0.15s",
      }}
    >
      {translation || " "}
    </div>
  );
}
