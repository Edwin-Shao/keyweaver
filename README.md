# KeyWeaver ⌨️🧵

自适应打字练习桌面应用，边打字边背单词。

An adaptive typing trainer with real-time Chinese translation — practice typing while learning vocabulary.

<p align="center">
  <img src="src-tauri/icons/icon.png" width="128" alt="KeyWeaver">
</p>

---

## 下载 / Download

前往 [Releases](https://github.com/Edwin-Shao/keyweaver/releases) 下载最新版本：

| 平台 | 文件 |
|------|------|
| **macOS** (Apple Silicon) | `KeyWeaver_x.x.x_aarch64.dmg` |

或通过 Cargo 安装：

```bash
cargo install --git https://github.com/Edwin-Shao/keyweaver
```

需要 [Rust](https://rustup.rs/) 1.75+。

---

## 功能 / Features

- **自适应文本生成** — 基于英语语音 Markov 链生成练习文本，忠实复刻 keybr.com 算法
- **渐进式字母解锁** — 从 6 个字母开始，达标后自动解锁新字母（共 26 个）
- **100% 中文翻译覆盖** — 9906 个单词全部配有中文释义，光标所在单词下方实时显示
- **26 键置信度热力图** — 每个键的掌握程度一目了然，从红到绿渐变
- **实时统计** — WPM（每分钟词数）、准确率、得分，练习结束后显示总结
- **两种纠错模式** — Forgive（容错前进）/ Stop on Error（停在原地纠正）
- **进度追踪** — 历史成绩列表、每日练习目标进度条
- **自动备份** — 每次保存前自动备份旧存档，不怕崩档
- **配置灵活** — 目标速度、纠错模式、文本长度、真实单词开关均可调

---

## 玩法 / How to Use

1. 启动后点击 **Start Practice** 开始练习
2. 屏幕上显示一段英文文本，逐个字符输入
3. 正确 → 绿色，错误 → 红色，修正后 → 黄色
4. 光标所在单词的中文翻译显示在文字下方
5. 输入完所有字符后显示本轮成绩，自动进入下一轮
6. 当所有已解锁字母达标后，自动解锁新字母
7. 按 `Esc` 返回菜单，`Backspace` 修正，`Tab` 切换纠错模式

---

## 自适应算法 / Algorithm

1. **字母调度**：从 e, n, i, a, r, l 六个字母开始，追踪每个键的反应时间
2. **解锁机制**：所有已解锁字母达到目标速度后，按词频顺序解锁下一个
3. **聚焦键**：最弱的键在练习文本中出现频率最高，重点练习
4. **文本生成**：Markov 链生成可读伪单词 + 10000 真实英文单词词典混合
5. **追踪反馈**：每次击键时间被记录、滤波、平滑，实时更新置信度

---

## 技术栈 / Tech Stack

- **前端**：React 19 + TypeScript + Vite + Zustand
- **后端**：Rust + Tauri v2
- **引擎**：移植自 [keybr-tui](https://github.com/Edwin-Shao/keybr-tui)（Markov 链 + 字母调度 + 词典）
- **词典**：ECDICT（英汉词典）+ MyMemory API 补充

---

## 开发 / Development

```bash
git clone https://github.com/Edwin-Shao/keyweaver.git
cd keyweaver
npm install
npm run tauri dev
```

构建：

```bash
npm run tauri build
# macOS DMG 输出在 src-tauri/target/release/bundle/dmg/
```

---

## 致谢 / Credits

- 算法源自 [keybr.com](https://www.keybr.com) by [aradzie](https://github.com/aradzie/keybr.com)
- 打字引擎移植自 [y0sif/keybr-tui](https://github.com/y0sif/keybr-tui)
- 中文词典 [ECDICT](https://github.com/skywind3000/ECDICT)

## License

[MIT](LICENSE)
