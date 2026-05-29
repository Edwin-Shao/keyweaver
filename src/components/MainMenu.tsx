import { useEffect } from "react";
import { useAppStore } from "../stores/appStore";

export default function MainMenu() {
  const { setScreen, startLesson, loadSettings } = useAppStore();

  useEffect(() => {
    loadSettings();
  }, []);

  return (
    <div
      style={{
        height: "100%",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        gap: 24,
        userSelect: "none",
      }}
    >
      <h1
        style={{
          fontSize: 48,
          fontWeight: 700,
          letterSpacing: -2,
          color: "#fff",
          marginBottom: 32,
        }}
      >
        KeyWeaver
      </h1>

      <button
        onClick={() => startLesson()}
        style={buttonStyle}
        onMouseEnter={(e) => (e.currentTarget.style.background = "#4a5acf")}
        onMouseLeave={(e) => (e.currentTarget.style.background = "#3b4cc0")}
      >
        Start Practice
      </button>

      <button
        onClick={() => setScreen("progress")}
        style={{ ...buttonStyle, background: "#2a2a3a" }}
        onMouseEnter={(e) => (e.currentTarget.style.background = "#3a3a4a")}
        onMouseLeave={(e) => (e.currentTarget.style.background = "#2a2a3a")}
      >
        View Progress
      </button>

      <button
        onClick={() => setScreen("settings")}
        style={{ ...buttonStyle, background: "#2a2a3a" }}
        onMouseEnter={(e) => (e.currentTarget.style.background = "#3a3a4a")}
        onMouseLeave={(e) => (e.currentTarget.style.background = "#2a2a3a")}
      >
        Settings
      </button>
    </div>
  );
}

const buttonStyle: React.CSSProperties = {
  width: 280,
  padding: "16px 32px",
  fontSize: 18,
  fontWeight: 600,
  color: "#fff",
  background: "#3b4cc0",
  borderRadius: 10,
  transition: "background 0.15s",
};
