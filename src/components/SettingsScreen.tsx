import { useEffect, useState } from "react";
import { useAppStore } from "../stores/appStore";
import type { Settings } from "../types";

export default function SettingsScreen() {
  const { loadSettings, saveSettings, setScreen } = useAppStore();
  const [form, setForm] = useState<Settings | null>(null);

  useEffect(() => {
    loadSettings().then(() => {
      const s = useAppStore.getState().settings;
      if (s) setForm({ ...s });
    });
  }, []);

  if (!form) {
    return (
      <div style={{ height: "100%", display: "flex", alignItems: "center", justifyContent: "center", color: "#666" }}>
        Loading...
      </div>
    );
  }

  const update = (patch: Partial<Settings>) => setForm((f) => f && { ...f, ...patch });

  return (
    <div style={{ height: "100%", padding: "40px 48px" }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: 32 }}>
        <h2 style={{ fontSize: 28, fontWeight: 700, color: "#fff" }}>Settings</h2>
        <button
          onClick={() => setScreen("menu")}
          style={{ padding: "8px 20px", fontSize: 14, color: "#999", background: "#2a2a3a", borderRadius: 8 }}
        >
          Back
        </button>
      </div>

      <div style={{ display: "flex", flexDirection: "column", gap: 24, maxWidth: 400 }}>
        <SettingRow label="Target WPM">
          <button onClick={() => update({ targetWpm: Math.max(10, form.targetWpm - 5) })} style={adjBtn}>−</button>
          <span style={{ width: 48, textAlign: "center", fontSize: 20, fontWeight: 600 }}>{form.targetWpm}</span>
          <button onClick={() => update({ targetWpm: Math.min(200, form.targetWpm + 5) })} style={adjBtn}>+</button>
        </SettingRow>

        <SettingRow label="Error Mode">
          <button
            onClick={() => update({ errorMode: form.errorMode === "forgive" ? "stop" : "forgive" })}
            style={{ padding: "6px 16px", fontSize: 14, color: "#fff", background: "#3b4cc0", borderRadius: 6 }}
          >
            {form.errorMode === "forgive" ? "Forgive" : "Stop on Error"}
          </button>
        </SettingRow>

        <SettingRow label="Fragment Length">
          <button onClick={() => update({ fragmentLength: Math.max(20, form.fragmentLength - 10) })} style={adjBtn}>−</button>
          <span style={{ width: 48, textAlign: "center", fontSize: 20, fontWeight: 600 }}>{form.fragmentLength}</span>
          <button onClick={() => update({ fragmentLength: Math.min(500, form.fragmentLength + 10) })} style={adjBtn}>+</button>
        </SettingRow>

        <SettingRow label="Natural Words">
          <button
            onClick={() => update({ naturalWords: !form.naturalWords })}
            style={{ padding: "6px 16px", fontSize: 14, color: "#fff", background: form.naturalWords ? "#4ade80" : "#666", borderRadius: 6 }}
          >
            {form.naturalWords ? "On" : "Off"}
          </button>
        </SettingRow>

        <SettingRow label="Daily Goal (min)">
          <button onClick={() => update({ dailyGoalMinutes: Math.max(0, form.dailyGoalMinutes - 5) })} style={adjBtn}>−</button>
          <span style={{ width: 48, textAlign: "center", fontSize: 20, fontWeight: 600 }}>{form.dailyGoalMinutes}</span>
          <button onClick={() => update({ dailyGoalMinutes: Math.min(120, form.dailyGoalMinutes + 5) })} style={adjBtn}>+</button>
        </SettingRow>

        <button
          onClick={async () => {
            await saveSettings(form);
            setScreen("menu");
          }}
          style={{
            marginTop: 16,
            padding: "12px 32px",
            fontSize: 16,
            fontWeight: 600,
            color: "#fff",
            background: "#3b4cc0",
            borderRadius: 10,
            alignSelf: "flex-start",
          }}
        >
          Save & Back
        </button>
      </div>
    </div>
  );
}

function SettingRow({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
      <span style={{ fontSize: 15, color: "#aaa" }}>{label}</span>
      <div style={{ display: "flex", alignItems: "center", gap: 8 }}>{children}</div>
    </div>
  );
}

const adjBtn: React.CSSProperties = {
  width: 32, height: 32, fontSize: 18, fontWeight: 700,
  color: "#fff", background: "#2a2a3a", borderRadius: 6, border: "none", cursor: "pointer",
};
