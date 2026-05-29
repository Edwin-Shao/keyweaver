import { useAppStore } from "./stores/appStore";
import MainMenu from "./components/MainMenu";
import TypingScreen from "./components/TypingScreen";
import ProgressScreen from "./components/ProgressScreen";
import SettingsScreen from "./components/SettingsScreen";

function App() {
  const screen = useAppStore((s) => s.screen);

  switch (screen) {
    case "menu":
      return <MainMenu />;
    case "typing":
      return <TypingScreen />;
    case "progress":
      return <ProgressScreen />;
    case "settings":
      return <SettingsScreen />;
  }
}

export default App;
