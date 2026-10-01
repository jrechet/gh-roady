# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Development Commands

```bash
# Build
cargo build              # Debug build
cargo build --release    # Release build (optimized, stripped)

# Run
cargo run -- <command>   # Run with arguments (e.g., cargo run -- tui)

# Test
cargo test               # Run all tests
cargo test cli_tests     # Run CLI integration tests
cargo test --test cli_tests -- test_cli_help  # Run single test

# Lint & Format
cargo clippy             # Lint
cargo fmt                # Format code
```

## CI/CD disponibles

Le dépôt est hybride : il vit sur GitHub (référence) et sur la forge Forgejo
`https://forge.jrec.fr/jrechet/gh-roady` (suiveur).

- **GitHub Actions** (`.github/workflows/`) : `release.yml` seulement (tag `v*` : build
  Linux/macOS/Windows sur les runners GitHub et publication des binaires sur la release).
  Pas de CI de PR côté GitHub. Suivi : `gh run list`, `gh run view <id> --log-failed`.
- **Forge** : `.github/workflows/mirror-to-forge.yml` recopie chaque branche et tag sur la
  forge (secret `FORGE_TOKEN`, posé par le propriétaire). La forge n'exécute que
  `.forgejo/workflows/ci.yml` (`cargo fmt --check`, `clippy`, `build`, `test` sur
  `[self-hosted, jre-server]` ; `fmt` et `clippy` non bloquants tant qu'ils échouent sur
  `main`) : jamais de release. Ne pas y ajouter d'action propre à
  GitHub. Suivi : `ssh jrec.fr '~/dev/server-app/forgejo/forge-tool.sh runs gh-roady'`.
- **Un seul endroit publie** : GitHub (`release.yml`), tant qu'il reste la référence.

## Architecture

This is a Rust CLI/TUI tool (`ghr`) for managing GitHub Actions storage (artifacts/caches). It follows strict **Domain-Driven Design (DDD)** with 4 layers:

```
src/
├── domain/       # Core business logic (zero external dependencies)
│   ├── artifact.rs, repo.rs, user.rs, storage.rs  # Entities
│   ├── github.rs     # GithubRepository trait (interface)
│   └── error.rs      # Domain errors
├── app/          # Use cases (orchestration, depends only on domain)
│   ├── artifact_use_case.rs
│   ├── repo_use_case.rs
│   └── auth_use_case.rs
├── infra/        # Implementations (implements domain interfaces)
│   ├── github/   # GitHubClient, AuthManager (octocrab-based)
│   ├── config/   # storage.rs (keyring, config files)
│   └── utils/    # cache.rs
└── presenter/    # User interfaces (delegates to app layer)
    ├── cli/      # Clap-based commands (auth, ls, artifacts, df)
    └── tui/      # Ratatui interactive interface (app.rs, ui.rs, events.rs)
```

**Key constraint**: Layer dependencies flow downward only. Domain has no deps, App depends on Domain, Infra implements Domain interfaces, Presenter calls App.

## Key Patterns

- **Repository trait**: `domain::github::GithubRepository` defines all GitHub operations; `infra::github::client::GitHubClient` implements it
- **Mocking**: The trait uses `#[cfg_attr(test, mockall::automock)]` for unit testing
- **Auth flow**: `AuthManager` tries `gh auth token` first, then keyring, then env var `GITHUB_TOKEN`
- **TUI state machine**: `presenter::tui::app::App` manages views (MainMenu, RepoList, StorageManager, etc.) and async data loading

## CLI Structure

Binary name: `ghr`

Commands: `auth` (login/logout/status), `ls` (list repos), `artifacts` (list/delete), `df` (storage analysis), `tui` (interactive mode)

## Testing Approach

- Integration tests in `tests/cli_tests.rs` use `assert_cmd` to verify CLI behavior
- Unit tests use `mockall` to mock `GithubRepository` trait
- Empty `tests/unit/` and `tests/integration/` directories exist for future expansion
