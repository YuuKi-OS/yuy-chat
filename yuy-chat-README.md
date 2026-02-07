<div align="center">

<br>

<img src="https://img.shields.io/badge/%E2%9C%A6-YUY--CHAT-000000?style=for-the-badge&labelColor=000000" alt="Yuy Chat" height="50">

<br><br>

# Beautiful TUI Chat for Local AI Models

**Talk to Yuuki models in your terminal.**<br>
**Streaming responses. Conversation history. Zero cloud required.**

<br>

<a href="#quick-start"><img src="https://img.shields.io/badge/GET_STARTED-000000?style=for-the-badge" alt="Get Started"></a>
&nbsp;&nbsp;
<a href="https://huggingface.co/OpceanAI/Yuuki-best"><img src="https://img.shields.io/badge/MODELS-000000?style=for-the-badge" alt="Models"></a>
&nbsp;&nbsp;
<a href="https://github.com/YuuKi-OS/yuy"><img src="https://img.shields.io/badge/YUY_CLI-000000?style=for-the-badge" alt="Yuy CLI"></a>
&nbsp;&nbsp;
<a href="https://huggingface.co/spaces/OpceanAI/Yuuki"><img src="https://img.shields.io/badge/LIVE_DEMO-000000?style=for-the-badge" alt="Demo"></a>

<br><br>

[![License](https://img.shields.io/badge/Apache_2.0-222222?style=flat-square&logo=apache&logoColor=white)](LICENSE)
&nbsp;
[![Rust](https://img.shields.io/badge/Rust_2021-222222?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
&nbsp;
[![ratatui](https://img.shields.io/badge/ratatui_0.26-222222?style=flat-square&logo=rust&logoColor=white)](https://github.com/ratatui-org/ratatui)
&nbsp;
[![Tokio](https://img.shields.io/badge/Tokio_Async-222222?style=flat-square&logo=rust&logoColor=white)](https://tokio.rs/)
&nbsp;
[![Termux](https://img.shields.io/badge/Termux-222222?style=flat-square&logo=android&logoColor=white)](#platform-support)
&nbsp;
[![Linux](https://img.shields.io/badge/Linux-222222?style=flat-square&logo=linux&logoColor=white)](#platform-support)
&nbsp;
[![macOS](https://img.shields.io/badge/macOS-222222?style=flat-square&logo=apple&logoColor=white)](#platform-support)
&nbsp;
[![Windows](https://img.shields.io/badge/Windows-222222?style=flat-square&logo=windows&logoColor=white)](#platform-support)

<br>

---

<br>

<table>
<tr>
<td width="55%" valign="top">

```
+--------------------------------------------------+
| yuy-chat v0.1.0        Yuuki-best | Balanced     |
+--------------------------------------------------+
|                                                  |
|  You: Explain async/await in Rust                |
|                                                  |
|  Yuuki: async/await in Rust allows you to write  |
|  asynchronous code that looks synchronous. The   |
|  async keyword marks a function as returning a   |
|  Future, and await suspends execution until the  |
|  Future resolves...                              |
|                                                  |
+--------------------------------------------------+
| Message: _                                       |
+--------------------------------------------------+
| Enter: Send | Ctrl+C: Menu | Ctrl+S: Save       |
+--------------------------------------------------+
```

</td>
<td width="45%" valign="top">

**A full chat experience in your terminal.**

<br>

Select models interactively.<br>
Stream responses word by word.<br>
Save and reload conversations.<br>
Switch presets on the fly.<br>
<br>
All running locally on your machine.<br>
All powered by ratatui + Rust.

</td>
</tr>
</table>

<br>

</div>

---

<br>

<div align="center">

## What is yuy-chat?

</div>

<br>

**yuy-chat** is a terminal user interface (TUI) application for chatting with local AI models. Built with Rust and powered by [ratatui](https://github.com/ratatui-org/ratatui), it provides a polished, keyboard-driven interface for real-time conversations with Yuuki models -- without ever leaving the terminal.

It connects to proven inference backends (**llama.cpp** and **llamafile**) and optionally to the **HuggingFace Inference API** for cloud-based generation. Model discovery, conversation management, preset switching, and streaming are all handled out of the box.

yuy-chat is the companion tool to [yuy](https://github.com/YuuKi-OS/yuy) (the CLI for downloading and managing models). Together they form the complete local inference toolkit for the Yuuki project.

<br>

---

<br>

<div align="center">

## Features

</div>

<br>

<table>
<tr>
<td width="50%" valign="top">

<h3>Interactive Chat</h3>

Real-time streaming responses displayed word by word. Multi-line input with Shift+Enter. Scrollable message history with keyboard navigation.

<br>

<h3>Model Selector</h3>

Auto-discovers `.gguf` and `.llamafile` models from your local directory. Navigate with arrow keys, select with Enter. Refresh without restarting.

<br>

<h3>Conversation History</h3>

Save conversations as JSON files. Load previous chats from a built-in conversation browser. Delete old sessions you no longer need.

<br>

<h3>HuggingFace Cloud</h3>

Optional API integration for cloud-based inference. Configure your HF token in the settings screen. Local and cloud models appear side by side in the selector.

</td>
<td width="50%" valign="top">

<h3>Generation Presets</h3>

Three built-in modes -- Creative (0.8 temp), Balanced (0.6 temp), and Precise (0.3 temp). Cycle between them with a single keypress. Custom presets planned for v0.2.

<br>

<h3>Settings Screen</h3>

Configure models directory, HuggingFace token, default preset, history saving, and UI theme -- all from within the TUI.

<br>

<h3>Cross-Platform</h3>

Runs on Termux (Android), Linux, macOS, and Windows. Same binary, same interface, same experience. Mobile-first defaults for constrained hardware.

<br>

<h3>Lightweight</h3>

~8 MB binary. ~20 MB idle RAM. ~50 ms startup. Built with Rust for zero-overhead performance and memory safety.

</td>
</tr>
</table>

<br>

---

<br>

<div align="center">

## Installation

</div>

<br>

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or later (1.75+ recommended)
- An inference runtime: [llama.cpp](https://github.com/ggerganov/llama.cpp) or a `.llamafile` model
- AI models in GGUF or Llamafile format (use [yuy](https://github.com/YuuKi-OS/yuy) to download them)

<br>

### From Source

```bash
git clone https://github.com/YuuKi-OS/yuy-chat
cd yuy-chat
cargo build --release
cargo install --path .
```

<br>

### Termux (Android)

```bash
pkg install rust git
git clone https://github.com/YuuKi-OS/yuy-chat
cd yuy-chat
cargo build --release -j 1
cargo install --path .
```

> **Note:** First compilation takes longer on ARM due to CPU constraints. Use `-j 1` to avoid thermal throttling. Incremental builds are fast (~10 sec).

<br>

### Verify

```bash
yuy-chat --version
```

<br>

---

<br>

<div align="center">

## Quick Start

</div>

<br>

```bash
# 1. Get a model (using yuy CLI)
yuy download Yuuki-best

# 2. Install a runtime
pkg install llama-cpp        # Termux
brew install llama.cpp       # macOS

# 3. Launch the chat
yuy-chat
```

The interface opens in the model selector. Pick a model with arrow keys and press Enter. Start typing and hit Enter to send messages. That's it.

<br>

---

<br>

<div align="center">

## Keyboard Reference

</div>

<br>

### Model Selector

| Key | Action |
|:----|:-------|
| `Up` / `k` | Previous model |
| `Down` / `j` | Next model |
| `Enter` | Select model |
| `R` | Refresh model list |
| `Q` | Quit |

<br>

### Chat

| Key | Action |
|:----|:-------|
| `Enter` | Send message |
| `Shift+Enter` | New line |
| `Ctrl+Enter` | Force send |
| `Up` / `Down` | Scroll history (when input is empty) |
| `Ctrl+C` | Open menu |
| `Ctrl+L` | Clear chat |
| `Ctrl+S` | Save conversation |
| `Backspace` | Delete character |

<br>

### Menu

| Key | Action |
|:----|:-------|
| `1` | Change model |
| `2` | Cycle preset |
| `3` | Save conversation |
| `4` | Load conversation |
| `5` | Clear chat |
| `6` | Settings |
| `Q` / `Esc` | Back to chat |

<br>

### Settings

| Key | Action |
|:----|:-------|
| `Up` / `Down` | Navigate settings |
| `Enter` | Edit setting |
| `Esc` | Back to menu |

<br>

---

<br>

<div align="center">

## Generation Presets

</div>

<br>

| Preset | Temperature | Top P | Best For |
|:-------|:------------|:------|:---------|
| **Creative** | 0.8 | 0.9 | Stories, brainstorming, creative writing |
| **Balanced** | 0.6 | 0.7 | General chat, explanations **(default)** |
| **Precise** | 0.3 | 0.5 | Code, math, factual answers |

Cycle presets during a chat session via `Ctrl+C` then `2`, or set a default in the configuration file.

<br>

---

<br>

<div align="center">

## Supported Formats and Runtimes

</div>

<br>

<table>
<tr>
<td width="50%" valign="top">

<h3>Model Formats</h3>

| Format | Extension | Notes |
|:-------|:----------|:------|
| GGUF | `.gguf` | Recommended. Requires llama.cpp |
| Llamafile | `.llamafile` | Self-executing. Zero dependencies |

</td>
<td width="50%" valign="top">

<h3>Inference Runtimes</h3>

| Runtime | Type | Notes |
|:--------|:-----|:------|
| llama.cpp | Local subprocess | Default. Fast, CPU-optimized |
| llamafile | Local executable | Bundled runtime + model |
| HuggingFace API | Cloud HTTP | Optional. Requires token |

</td>
</tr>
</table>

<br>

yuy-chat auto-detects the appropriate runtime based on the selected model's format. For GGUF models, it searches for `llama-cli`, `llama`, or `main` binaries in PATH.

<br>

---

<br>

<div align="center">

## HuggingFace Integration

</div>

<br>

Cloud-based inference is optional. To enable it:

1. Get a token from [huggingface.co/settings/tokens](https://huggingface.co/settings/tokens)
2. Open Settings in yuy-chat (`Ctrl+C` then `6`)
3. Navigate to "HuggingFace Token" and paste it

Cloud models then appear in the selector alongside local models:

```
> Yuuki-best-q5.gguf          2.3 GB  [Local]
  Yuuki-3.7-q4.gguf           1.8 GB  [Local]
  Yuuki-best (via API)        Cloud   [HuggingFace]
```

| | Local | Cloud |
|:--|:------|:------|
| Speed | Faster (no network) | Depends on connection |
| Privacy | 100% offline | Data sent to HF API |
| Storage | Requires disk space | None |
| Availability | Always | Requires internet |

<br>

---

<br>

<div align="center">

## Configuration

</div>

<br>

### Config File

Location: `~/.config/yuy-chat/config.toml`

```toml
models_dir = "/home/user/.yuuki/models"
default_preset = "Balanced"
save_history = true
theme = "Dark"

# Optional
# hf_token = "hf_xxxxxxxxxxxxx"
```

### Priority Order

1. **TUI settings** -- changes made in the settings screen
2. **Config file** -- `~/.config/yuy-chat/config.toml`
3. **Defaults** -- sensible defaults based on platform detection

<br>

### Directory Layout

```
~/.config/yuy-chat/
    config.toml                     # configuration
    conversations/                  # saved chats
        conversation-20260206-143022.json
        conversation-20260206-150133.json

~/.yuuki/models/                    # models (shared with yuy CLI)
    Yuuki-best/
        yuuki-best-q4_0.gguf
    Yuuki-3.7/
        yuuki-3.7-q5_k_m.gguf
```

<details>
<summary><strong>Platform-specific paths</strong></summary>
<br>

| Platform | Config path |
|:---------|:------------|
| Linux | `~/.config/yuy-chat/config.toml` |
| macOS | `~/.config/yuy-chat/config.toml` |
| Windows | `C:\Users\{user}\AppData\Roaming\yuy-chat\config.toml` |
| Termux | `/data/data/com.termux/files/home/.config/yuy-chat/config.toml` |

</details>

<br>

---

<br>

<div align="center">

## Architecture

</div>

<br>

```
                              User
                                |
                                v
  +-------------------------------------------------------------+
  |                      yuy-chat (Rust)                        |
  |                                                             |
  |   main.rs              Entry point + event loop             |
  |       |                crossterm polling (100ms)             |
  |       v                                                     |
  |   app.rs               State machine                        |
  |       |                ModelSelector | Chat | Menu |        |
  |       |                Settings | ConversationList          |
  |       v                                                     |
  |   ui/                  Rendering layer (ratatui)            |
  |       |                selector.rs, chat.rs, menu.rs,       |
  |       |                settings.rs, conversations.rs        |
  |       v                                                     |
  |   models/              Business logic                       |
  |                        scanner.rs, runtime.rs, hf_api.rs    |
  +-------+----------------------------+-----------------------+
          |                            |
          v                            v
  +------------------+       +-------------------+
  |  External APIs   |       |  Local Storage    |
  |  HuggingFace     |       |  ~/.config/       |
  |  Inference API   |       |  ~/.yuuki/models/ |
  +--------+---------+       +-------------------+
           |
           v
  +--------------------------------+
  |      Inference Runtimes        |
  |  llama.cpp  |  llamafile       |
  +--------------------------------+
```

<br>

### Source Layout

```
yuy-chat/
    Cargo.toml                  # manifest and dependencies
    README.md
    TECHNICAL.md                # full technical specification
    src/
        main.rs                 # entry point, event loop, terminal setup
        app.rs                  # application state, message handling
        config.rs               # config load/save, presets, themes
        conversation.rs         # message storage, JSON persistence
        models/
            mod.rs              # module declarations
            scanner.rs          # auto-discovery of local + HF models
            runtime.rs          # subprocess management, streaming
            hf_api.rs           # HuggingFace Inference API client
        ui/
            mod.rs              # module declarations
            selector.rs         # model selection screen
            chat.rs             # main chat interface
            menu.rs             # options menu
            settings.rs         # configuration screen
            conversations.rs    # saved conversations browser
```

<br>

### Data Flow

```
User Input --> Event Loop --> App State --> Business Logic --> UI Render
                  ^                             |
                  +-----------------------------+
                       (state mutation loop)
```

<br>

### Design Patterns

| Pattern | Implementation |
|:--------|:---------------|
| State machine | `AppState` enum drives which screen is active and how events are routed |
| Async streaming | Tokio channels (`mpsc`) pipe inference output chunk-by-chunk to the UI |
| Subprocess isolation | llama.cpp runs in a spawned `Child` process with piped stdout |
| Double buffering | ratatui handles minimal redraws automatically |
| Lazy loading | Models and conversations are loaded on-demand, not at startup |

<br>

---

<br>

<div align="center">

## Technical Specifications

</div>

<br>

### Project Metrics

```
Language:              Rust 2021 Edition
Lines of code:         ~1,453
Rust source files:     15
Modules:               5
Public functions:      ~45
Data structures:       12
Enums:                 8
Direct dependencies:   16
Binary size (release): ~8 MB
```

<br>

### Performance

| Operation | Time |
|:----------|:-----|
| Startup (no models) | ~50 ms |
| Startup (10 models) | ~200 ms |
| Model scan (10 models) | ~100 ms |
| Render frame | ~1-2 ms |
| Send message (pre-inference) | ~5 ms |
| Save conversation | ~10 ms |

<br>

### Memory Usage

| State | RAM |
|:------|:----|
| Idle (no model) | ~20 MB |
| Model loaded | ~50 MB |
| Active inference | ~100-500 MB |
| Peak (large models) | ~1 GB |

<br>

### System Requirements

| Requirement | Minimum | Recommended |
|:------------|:--------|:------------|
| Rust | 1.70+ | 1.75+ |
| RAM | 512 MB | 2 GB |
| Disk | 50 MB (binary) | 100 MB |
| CPU | ARM/x86 (32/64-bit) | x86_64 or ARM64 |
| Terminal | Unicode support | Modern terminal emulator |

<br>

### Dependencies

| Crate | Purpose |
|:------|:--------|
| `ratatui` | Terminal UI framework |
| `crossterm` | Cross-platform terminal control |
| `tokio` | Async runtime |
| `reqwest` | HTTP client (HuggingFace API) |
| `serde` + `serde_json` + `toml` | Serialization |
| `chrono` | Timestamps for conversations |
| `walkdir` | Recursive model directory scanning |
| `dirs` | Cross-platform home directory |
| `anyhow` + `thiserror` | Error handling |
| `colored` | Terminal colors |
| `tracing` | Logging |
| `which` | Binary detection in PATH |

<br>

---

<br>

<div align="center">

## Platform Support

</div>

<br>

| Platform | Status | Notes |
|:---------|:-------|:------|
| **Termux (Android)** | Full support | Primary target. ARM64 tested |
| **Linux x86_64** | Full support | Ubuntu 22.04+ tested |
| **Linux ARM64** | Full support | Raspberry Pi 4 tested |
| **macOS Intel** | Full support | Catalina+ tested |
| **macOS Apple Silicon** | Full support | M1/M2 tested |
| **Windows 10/11** | Full support | Windows 11 tested |
| **FreeBSD** | Untested | Should work |

<br>

<details>
<summary><strong>Termux (Android) -- Primary Target</strong></summary>
<br>

Optimizations applied automatically when Termux is detected:

- Single-threaded compilation (`-j 1`) to prevent thermal throttling
- Conservative I/O patterns for mobile storage
- Simplified progress indicators for narrow terminal widths

Detection method:

```rust
std::env::var("PREFIX")
    .map(|p| p.contains("com.termux"))
    .unwrap_or(false)
```

</details>

<details>
<summary><strong>macOS</strong></summary>
<br>

- Metal GPU acceleration available through llama.cpp
- Homebrew for runtime installation (`brew install llama.cpp`)
- Full keyboard support in Terminal.app and iTerm2

</details>

<details>
<summary><strong>Windows</strong></summary>
<br>

- Windows Terminal recommended for best rendering
- Backslash path handling automatic
- CUDA acceleration via llama.cpp for NVIDIA GPUs

</details>

<br>

---

<br>

<div align="center">

## Conversation Format

</div>

<br>

Conversations are saved as JSON files in `~/.config/yuy-chat/conversations/`.

**Filename convention:** `conversation-{YYYYMMDD}-{HHMMSS}.json`

```json
{
  "messages": [
    {
      "role": "user",
      "content": "Explain async/await in Rust",
      "timestamp": "2026-02-06T14:30:22.123Z"
    },
    {
      "role": "assistant",
      "content": "async/await in Rust allows you to write...",
      "timestamp": "2026-02-06T14:30:25.456Z"
    }
  ],
  "created_at": "2026-02-06T14:30:22.123Z",
  "updated_at": "2026-02-06T14:35:10.789Z"
}
```

<br>

---

<br>

<div align="center">

## Security

</div>

<br>

### Current

- **HTTPS only** -- all HuggingFace API calls use TLS (rustls, no OpenSSL)
- **No shell injection** -- subprocesses use `Command::arg()`, never string interpolation
- **Scoped file access** -- all reads/writes within `~/.config/yuy-chat/` and `~/.yuuki/`
- **Process isolation** -- llama.cpp runs as a separate subprocess with piped I/O

### Known Limitations

- HuggingFace tokens are stored in plaintext in `config.toml`
- File permissions are not enforced on config files

### Planned (v0.2+)

- System keyring integration for token storage
- File permission enforcement (`0o600` for sensitive files)
- Encrypted token storage on Termux via libsodium
- Input size limits and sanitization

<br>

---

<br>

<div align="center">

## Troubleshooting

</div>

<br>

<details>
<summary><strong>No models found</strong></summary>
<br>

```bash
# Check if models exist
ls ~/.yuuki/models/

# Download a model using yuy CLI
yuy download Yuuki-best

# Or place a .gguf file manually
cp your-model.gguf ~/.yuuki/models/
```

</details>

<details>
<summary><strong>llama.cpp not found</strong></summary>
<br>

```bash
# Termux
pkg install llama-cpp

# macOS
brew install llama.cpp

# Verify
which llama-cli
```

</details>

<details>
<summary><strong>Permission denied on llamafile</strong></summary>
<br>

```bash
chmod +x ~/.yuuki/models/*.llamafile
```

</details>

<details>
<summary><strong>Slow responses</strong></summary>
<br>

- Use a smaller quantization (`q4_0` instead of `q8_0`)
- Check available RAM (`free -h` or `top`)
- Switch to the **Precise** preset (shorter outputs)
- Ensure no other heavy processes are running

</details>

<details>
<summary><strong>UI rendering issues</strong></summary>
<br>

- Use a terminal with Unicode support
- On Windows, use Windows Terminal (not CMD)
- On Termux, ensure terminal encoding is UTF-8
- Try resizing the terminal window

</details>

<br>

---

<br>

<div align="center">

## Roadmap

</div>

<br>

### v0.2 -- Enhanced UX

- [ ] Syntax highlighting for code blocks
- [ ] Copy/paste support
- [ ] Export conversations to Markdown
- [ ] Custom system prompts
- [ ] Vim keybindings mode
- [ ] Custom user-defined presets

### v0.3 -- Power Features

- [ ] Multiple chat tabs
- [ ] Search in conversation history
- [ ] Token usage statistics
- [ ] Model comparison mode
- [ ] Template system for prompts

### v1.0 -- Ecosystem

- [ ] Plugin system
- [ ] Custom themes (user-defined color schemes)
- [ ] Conversation branching
- [ ] Multi-modal support (images)
- [ ] REST API server mode

<br>

---

<br>

<div align="center">

## Contributing

</div>

<br>

### Development Setup

```bash
git clone https://github.com/YuuKi-OS/yuy-chat
cd yuy-chat

# install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# build and verify
cargo build
cargo test
cargo fmt -- --check
cargo clippy
```

### Commit Convention

```
<type>(<scope>): <subject>
```

Types: `feat` | `fix` | `docs` | `style` | `refactor` | `test` | `chore`

```
feat(chat): add multi-line input support

- Detect Shift+Enter for newlines
- Update input rendering for wrapped text
- Add cursor position tracking

Closes #12
```

### Pull Request Checklist

- [ ] Tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated if needed
- [ ] Commits follow the convention above

### Coding Standards

- `snake_case` for functions, `CamelCase` for types
- Document all public functions with `///` comments
- Use `Result<T>` and the `?` operator for error handling
- Prefer `async/await` over callbacks
- Justify any new dependency in the PR description

<br>

---

<br>

<div align="center">

## Design Decisions

</div>

<br>

<details>
<summary><strong>Why a TUI instead of a GUI or web UI?</strong></summary>
<br>

The primary target is Termux on Android. A TUI requires no display server, no browser, and minimal resources. It also works over SSH, inside tmux, and in any terminal emulator on any platform. A GUI may be added as an optional feature later.

</details>

<details>
<summary><strong>Why ratatui?</strong></summary>
<br>

ratatui is the most actively maintained TUI framework in the Rust ecosystem. It provides immediate-mode rendering, a rich widget library, and cross-platform terminal support through crossterm. The API is well-documented and the community is responsive.

</details>

<details>
<summary><strong>Why subprocess spawning instead of library linking?</strong></summary>
<br>

Linking llama.cpp as a C library adds significant build complexity, especially for cross-compilation and Termux. Spawning a subprocess is simpler, isolates crashes, and allows the user to update llama.cpp independently. Library integration is planned for v1.0.

</details>

<details>
<summary><strong>Why Tokio for a TUI?</strong></summary>
<br>

Inference is slow. Without async, the UI would freeze during response generation. Tokio enables non-blocking subprocess reads, smooth streaming display, and sets the foundation for future parallel features like multi-tab chat.

</details>

<details>
<summary><strong>Why JSON for conversations instead of SQLite?</strong></summary>
<br>

JSON files are human-readable, trivially portable, and require no additional dependency. Each conversation is self-contained. SQLite may be introduced in v1.0 if search and indexing become necessary.

</details>

<br>

---

<br>

<div align="center">

## Build Configuration

</div>

<br>

### Release Profile

```toml
[profile.release]
opt-level = "z"         # optimize for binary size
lto = true              # link-time optimization
codegen-units = 1       # single codegen unit for better optimization
strip = true            # strip debug symbols
```

### Environment Variables

```bash
RUST_LOG=debug yuy-chat          # enable debug logging
RUST_LOG=info yuy-chat           # info-level logging
YUY_MODELS_DIR=/path yuy-chat    # custom models directory
XDG_CONFIG_HOME=/custom yuy-chat # custom config directory
```

### Cross-Compilation

```bash
# ARM64 (Raspberry Pi, Termux native)
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu

# Windows (from Linux)
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# macOS Apple Silicon (from Linux)
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

<br>

---

<br>

<div align="center">

## About the Yuuki Project

</div>

<br>

yuy-chat exists to serve the [Yuuki project](https://huggingface.co/OpceanAI/Yuuki-best) -- a code-generation LLM being trained entirely on a smartphone with zero cloud budget.

<table>
<tr>
<td width="50%" valign="top">

**Training Details**

| | |
|:--|:--|
| Base model | GPT-2 (124M parameters) |
| Training type | Continued pre-training |
| Hardware | Snapdragon 685, CPU only |
| Training time | 50+ hours |
| Progress | 2,000 / 37,500 steps (5.3%) |
| Cost | $0.00 |

</td>
<td width="50%" valign="top">

**Quality Scores (Checkpoint 2000)**

| Language | Score |
|:---------|:------|
| Agda | 55 / 100 |
| C | 20 / 100 |
| Assembly | 15 / 100 |
| Python | 8 / 100 |

</td>
</tr>
</table>

A fully native model (trained from scratch, not fine-tuned) is planned for v1.0. A research paper documenting the mobile training methodology is in preparation.

<br>

---

<br>

<div align="center">

## Related Projects

</div>

<br>

| Project | Description |
|:--------|:------------|
| [yuy](https://github.com/YuuKi-OS/yuy) | CLI for downloading, managing, and running Yuuki models |
| [Yuuki-best](https://huggingface.co/OpceanAI/Yuuki-best) | Best checkpoint model weights |
| [Yuuki Space](https://huggingface.co/spaces/OpceanAI/Yuuki) | Web-based interactive demo |
| [yuuki-training](https://github.com/YuuKi-OS/yuuki-training) | Training code and scripts |

<br>

---

<br>

<div align="center">

## Links

</div>

<br>

<div align="center">

[![Model Weights](https://img.shields.io/badge/Model_Weights-Hugging_Face-ffd21e?style=for-the-badge&logo=huggingface&logoColor=black)](https://huggingface.co/OpceanAI/Yuuki-best)
&nbsp;
[![Live Demo](https://img.shields.io/badge/Live_Demo-Spaces-ffd21e?style=for-the-badge&logo=huggingface&logoColor=black)](https://huggingface.co/spaces/OpceanAI/Yuuki)
&nbsp;
[![Yuy CLI](https://img.shields.io/badge/Yuy_CLI-GitHub-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/YuuKi-OS/yuy)

<br>

[![Training Code](https://img.shields.io/badge/Training_Code-GitHub-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/YuuKi-OS/yuuki-training)
&nbsp;
[![Report Issue](https://img.shields.io/badge/Report_Issue-GitHub-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/YuuKi-OS/yuy-chat/issues)

</div>

<br>

---

<br>

<div align="center">

## License

</div>

<br>

```
Copyright 2026 Yuuki Project

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

<br>

---

<br>

<div align="center">

**Built with patience, a phone, and zero budget.**

<br>

[![Yuuki Project](https://img.shields.io/badge/Yuuki_Project-2026-000000?style=for-the-badge)](https://huggingface.co/OpceanAI)

<br>

</div>
