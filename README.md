# gh-roady (ghr) 🚀

A modern, fast, and powerful GitHub CLI companion with a beautiful Terminal User Interface. `ghr` helps you manage your repositories and optimize your GitHub Actions storage usage with ease.

Built with **Rust**, **Ratatui**, and **Octocrab**.

## 🌟 Key Features

- 🔐 **Secure Authentication**: Native integration with system keyring (Keychain, Secret Service, KWallet) or environment variables.
- 📊 **Storage Management (`df`)**: Interactive storage usage analysis. Find and delete large artifacts and caches across all your repos to free up shared storage.
- 📚 **Repository Exploration**: Live filtering and searching through your repositories and organizations.
- 🖥️ **Interactive TUI**: A premium terminal interface for browsing your GitHub resources.
- ⚡ **Blazing Fast**: Compiled Rust performance with minimal resource footprint.

## 🚀 Installation

### Download Binaries
You can download the latest pre-compiled binaries for macOS, Linux, and Windows from the [Releases](https://github.com/jrechet/gh-roady/releases) page.

### Build from source
If you have Rust installed, you can build it yourself:

```bash
git clone https://github.com/jrechet/gh-roady.git
cd gh-roady
cargo build --release
```

The binary will be available in `target/release/ghr`.

## 📖 Usage Guide

### 1. Authentication
First, provide your GitHub Personal Access Token (PAT). `ghr` will securely store it in your system's keyring.

```bash
# Interactive login
ghr auth login --token <YOUR_TOKEN>
```
*Tip: You can also set `GITHUB_TOKEN` in a `.env` file or as an environment variable.*

### 2. Manage Storage Usage (`df`)
This is the flagship feature for maintaining your GitHub Actions quota.

```bash
ghr df
```
- **Analyze**: See a visual bar of your shared storage usage vs. your plan limit.
- **Detail**: Lists all artifacts and caches, sorted by size.
- **Clean**: Select items using the **Space bar** and press **Enter** to delete them instantly.

### 3. Quick Lists
```bash
# List your repositories
ghr ls

# List artifacts for a specific repo
ghr artifacts list --owner <USER> --repo <REPO>

# Cleanup artifacts older than 30 days
ghr artifacts delete --older-than 30
```

### 4. Full TUI Mode
For a more immersive experience, launch the TUI:
```bash
ghr tui
```

#### Controls:
- **↑ / ↓** or **j / k**: Navigate list
- **Enter**: View repository details
- **/**: Start filtering/searching
- **Esc**: Go back / Stop searching
- **r**: Refresh data
- **q**: Quit

## 📝 License

Distributed under the MIT License. See `LICENSE` for more information.

---
Created by [jrechet](https://github.com/jrechet)
