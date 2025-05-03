# fgit

**fgit** is a Rust CLI tool that scans your desktop directory for Git repositories, caches the results for faster repeated runs, and lets you interactively select a repository using a simple prompt.

---

## ✨ Features

- 🔍 **Scan for Git repositories** under `~/Desktop`
- 🚀 **Parallel directory scanning** using Rayon for speed
- 💾 **Caching** of scan results to avoid unnecessary rescans
- 🗂 **Ignore common folders** like `node_modules`, `ios`, and `android`
- 🎛 **Interactive selector** (powered by `inquire`) to choose a repository
- 📅 **Sort by modification date** (newest first)

---

## 📦 Installation

1. **Clone the repository:**

```bash
git clone https://github.com/yourusername/fgit.git
cd fgit
```

2. **Build with Cargo**

```bash
cargo build --release
```

3. **Run the binary**

```bash
./target/release/fgit
```
