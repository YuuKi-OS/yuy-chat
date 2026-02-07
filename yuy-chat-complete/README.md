# yuy-chat

<div align="center">

```
$$\     $$\                    
\$$\   $$  |                   
 \$$\ $$  /$$\   $$\ $$\   $$\ 
  \$$$$  / $$ |  $$ |$$ |  $$ |
   \$$  /  $$ |  $$ |$$ |  $$ |
    $$ |   $$ |  $$ |$$ |  $$ |
    $$ |   \$$$$$$  |\$$$$$$$ |
    \__|    \______/  \____$$ |
                     $$\   $$ |
                     \$$$$$$  |
                      \______/ 
```

**Beautiful TUI chat interface for local AI models**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

</div>

---

## 🌟 Features

- ✨ **Beautiful TUI** - Gorgeous terminal interface powered by ratatui
- 🔍 **Auto-discovery** - Automatically finds `.gguf` and `.llamafile` models
- 🎨 **Presets** - Creative, Balanced, and Precise modes
- 💾 **Save conversations** - Keep your chat history
- 🌐 **HuggingFace API** - Use models from HuggingFace (optional)
- ⚡ **Fast & Lightweight** - ~5MB binary, minimal dependencies
- 🚀 **Streaming responses** - See words appear as they're generated
- 🎯 **Zero configuration** - Just run and chat

## 📦 Installation

### From source:

```bash
git clone https://github.com/YuuKi-OS/yuy-chat
cd yuy-chat
cargo build --release
```

### Install globally:

```bash
cargo install --path .
```

## 🚀 Quick Start

```bash
# Run yuy-chat
yuy-chat

# It will auto-scan ~/.yuuki/models/ for .gguf and .llamafile files
# Select a model and start chatting!
```

## 📁 Supported Model Formats

- ✅ **GGUF** (`.gguf`) - Runs with llama.cpp
- ✅ **Llamafile** (`.llamafile`) - Self-contained executables

## 🎮 Controls

### Model Selector
- `↑/↓` or `j/k` - Navigate models
- `Enter` - Select model
- `R` - Refresh model list
- `Q` - Quit

### Chat
- `Type` - Write your message
- `Enter` - Send message
- `Shift+Enter` - New line
- `Ctrl+Enter` - Send (always)
- `Ctrl+C` - Open menu
- `Ctrl+L` - Clear chat
- `Ctrl+S` - Save conversation
- `↑/↓` - Scroll chat (when input is empty)

### Menu
- `1` - Change model
- `2` - Change preset
- `3` - Save conversation
- `4` - Load conversation
- `5` - Clear chat
- `6` - Settings
- `Q` - Back to chat

## ⚙️ Configuration

Config file location: `~/.config/yuy-chat/config.toml`

```toml
models_dir = "/home/user/.yuuki/models"
hf_token = "hf_xxxxxxxxxxxxx"  # Optional
default_preset = "Balanced"
save_history = true
theme = "Dark"
```

## 🎯 Presets

- **Creative** (temp: 0.8, top_p: 0.9) - More random and creative
- **Balanced** (temp: 0.6, top_p: 0.7) - Good middle ground
- **Precise** (temp: 0.3, top_p: 0.5) - More focused and deterministic

## 🌐 HuggingFace Integration

Add your HuggingFace token in settings to use models via API:

1. Press `Ctrl+C` → `6` (Settings)
2. Edit `HuggingFace Token`
3. Paste your token from https://huggingface.co/settings/tokens
4. Save and refresh models

## 📚 Directory Structure

```
~/.config/yuy-chat/
├── config.toml              # Configuration
└── conversations/           # Saved chats
    ├── conversation-20240206-143022.json
    └── conversation-20240206-150133.json
```

## 🔧 Requirements

- **Rust 1.70+** (for building)
- **llama.cpp** (for .gguf models) - Install with: `yuy runtime install llama-cpp`
- **chmod +x** (for .llamafile models)

## 🤝 Integration with yuy

yuy-chat is designed to work alongside [yuy](https://github.com/YuuKi-OS/yuy):

```bash
# Download models with yuy
yuy download Yuuki-best

# Chat with yuy-chat
yuy-chat
```

## 🐛 Troubleshooting

**No models found?**
- Make sure you have models in `~/.yuuki/models/`
- Or specify custom directory: `yuy-chat --models-dir /path/to/models`

**llama.cpp not found?**
- Install with: `yuy runtime install llama-cpp`
- Or: `brew install llama.cpp` (macOS)
- Or: `pkg install llama-cpp` (Termux)

**Streaming not working?**
- Ensure llama.cpp is installed and in PATH
- Check model file permissions

## 📝 License

MIT License - see [LICENSE](LICENSE) file

## 🌸 Credits

Made with love by the Yuuki team

- TUI Framework: [ratatui](https://github.com/ratatui-org/ratatui)
- Inference: [llama.cpp](https://github.com/ggerganov/llama.cpp)

---

**For model management, see [yuy](https://github.com/YuuKi-OS/yuy)**
