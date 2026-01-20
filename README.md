# gh-roady (ghr) 🚀

A modern, fast, and powerful GitHub CLI companion with a beautiful Terminal User Interface. `ghr` helps you manage your repositories and optimize your GitHub Actions storage usage with ease.

Built with **Rust**, **Ratatui**, and **Octocrab**.

## 🌈 Why gh-roady?

If you've ever hit your GitHub Actions storage limit or struggled to find which repo is eating your 2GB quota, `ghr` is for you. It provides a visual, interactive way to clean up artifacts and caches across your entire account or organization.

## 🌟 Key Features

- 🔐 **Zero-Config Auth**: Works out-of-the-box with `gh auth login`. Native integration with system keyring or environment variables.
- 💾 **Storage Manager (`df`)**: Interactive storage usage analysis. Find and delete large artifacts and caches across ALL your repos instantly.
- 📚 **Repository Exploration**: Live filtering and searching through your repositories and organizations.
- 🖥️ **Interactive TUI**: A premium terminal interface with rainbow aesthetics, main menu navigation, and real-time loading indicators.
- ⚡ **Blazing Fast**: Compiled Rust performance with a minimal resource footprint.

## 🚀 Installation

### Download Binaries
Download the latest pre-compiled binary for your system from the [Releases](https://github.com/jrechet/gh-roady/releases) page:
- **macOS**: `ghr-macos-aarch64` (M1/M2/M3) or `ghr-macos-x86_64` (Intel)
- **Linux**: `ghr-linux-x86_64`
- **Windows**: `ghr-windows-x86_64.exe`

### Build from source
If you have Rust installed:

```bash
git clone https://github.com/jrechet/gh-roady.git
cd gh-roady
cargo build --release
```

## 📖 Usage Guide

### 1. Authentication
`ghr` is smart. If you are already logged in with the official GitHub CLI, it just works!

```bash
# Recommended: Login via GitHub CLI
gh auth login

# Or use ghr's own secure keyring
ghr auth login --token <YOUR_TOKEN>
```

### 2. Full TUI Mode (Recommended)
Launch the immersive experience:
```bash
ghr tui
```
Navigate between **Repositories**, **All Artifacts**, and the **Storage Manager** from the main menu.

#### TUI Controls:
- **↑ / ↓** or **j / k**: Navigate lists / menus
- **Enter**: Select menu item / View details
- **Space**: Toggle selection in Storage Manager
- **d**: Delete selected items (Storage Manager) or current artifact
- **/**: Start live filtering/searching
- **Esc**: Back to menu / Stop searching
- **r**: Refresh data
- **q**: Quit

### 3. CLI Storage Command (`df`)
Quickly check and clean your storage without entering the UI:

```bash
ghr df
```
- **Analyze**: Visual bar of shared storage usage (detects Pro vs. Free limits).
- **Detail**: Lists all items > 1KB, sorted by size.
- **Clean**: Interactive multi-select for deletion.

### 4. Other CLI Commands
```bash
# List your repositories
ghr ls

# List artifacts for a specific repo
ghr artifacts list --owner <USER> --repo <REPO>
```

## 📝 License

Distributed under the MIT License. See `LICENSE` for more information.

---
Created by [jrechet](https://github.com/jrechet)
