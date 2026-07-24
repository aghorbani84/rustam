# 🦀 Abolfazl Ghorbani - Portfolio

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Dioxus](https://img.shields.io/badge/Dioxus-000000?style=for-the-badge&logo=webassembly&logoColor=white)
![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)
![Tailwind CSS](https://img.shields.io/badge/Tailwind_CSS-38B2AC?style=for-the-badge&logo=tailwind-css&logoColor=white)

**Modern Portfolio Website built with Rust & Dioxus**

[🌐 Live Demo](https://aghorbani84.github.io/rustam/) •
[📦 Source Code](https://github.com/aghorbani84/rustam)

</div>

---

## ✨ Features

- 🎨 **Modern UI/UX** - Clean, professional design with smooth animations
- 🌓 **Dark Mode** - Automatic theme switching with system preference
- 📱 **Fully Responsive** - Mobile-first design with hamburger menu
- ⚡ **Lightning Fast** - Built with Rust and WebAssembly
- 🎯 **Multiple Apps** - Portfolio, Todo, Pomodoro Timer, Markdown Editor
- 💾 **Local Storage** - Persistent data across sessions
- 🚀 **Production Ready** - Optimized build with minimal bundle size
- 📱 **PWA Ready** - Installable as a native app

## 🛠️ Tech Stack

| Technology | Version | Purpose |
|------------|---------|---------|
| Rust | 2026 Edition | Core Language |
| Dioxus | 0.7 | UI Framework |
| Tailwind CSS | 3.x | Styling |
| WebAssembly | - | Runtime |
| Trunk | - | Build Tool |

## 📦 Projects Included

1. **Portfolio Website** - This website itself!
2. **Todo App** - Task management with filters and search
3. **Pomodoro Timer** - Focus timer with statistics
4. **Markdown Editor** - Real-time editor with preview

## 🚀 Getting Started

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Dioxus CLI
cargo install dioxus-cli
```

### Development

```bash
git clone https://github.com/abolfazlghorbani369/rustam.git
cd rustam
dx serve
```

Visit `http://localhost:8080`

### Production Build

```bash
dx build --release
```

## 🌐 Deployment

### GitHub Pages
- Push to `main` branch
- GitHub Actions auto-deploys
- Live at: `https://abolfazlghorbani369.github.io/rustam/`

### GitLab Pages
- Push to `main` branch
- GitLab CI auto-deploys
- Live at: `https://abolfazlghorbani369.gitlab.io/rustam/`

## 📁 Project Structure

```
src/
├── components/     # Reusable UI components
├── pages/          # Route-based pages
├── hooks/          # Custom hooks
├── state/          # Global state management
├── services/       # API calls
├── utils/          # Helper functions
├── models/         # Data models
├── styles/         # CSS styles
├── app.rs          # Main app component
└── main.rs         # Entry point
```

## 🔧 Build Optimization

```toml
[profile.release]
opt-level = "z"      # Optimize for size
lto = true           # Link Time Optimization
codegen-units = 1    # Single codegen unit
panic = "abort"      # Smaller binary
strip = true         # Remove debug symbols
```

## 📝 License

This project is licensed under **CC BY-NC-ND 4.0**:
- ✅ Copy with attribution
- ❌ No commercial use
- ❌ No derivative works

See [LICENSE](LICENSE) for details.

## 📧 Contact

**Abolfazl Ghorbani**
- GitHub: [@abolfazlghorbani369](https://github.com/abolfazlghorbani369)
- Email: abolfazlghorbani369@gmail.com

---

<div align="center">

**Built with ❤️ using Rust & Dioxus**

© 2026 Abolfazl Ghorbani. All rights reserved.

</div>
