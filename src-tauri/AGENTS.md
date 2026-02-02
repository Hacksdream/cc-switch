# Backend (Rust + Tauri 2)

## Overview

Tauri 2 backend providing native desktop functionality, SQLite database, HTTP proxy, and system integration.

## Directory Structure

| Path | Purpose | AGENTS.md |
|------|---------|-----------|
| `src/commands/` | Tauri command handlers | [src/commands/AGENTS.md](src/commands/AGENTS.md) |
| `src/services/` | Business logic services | [src/services/AGENTS.md](src/services/AGENTS.md) |
| `src/proxy/` | HTTP proxy implementation | [src/proxy/AGENTS.md](src/proxy/AGENTS.md) |
| `src/*.rs` | Core modules | — |

## Entry Points

- `main.rs` - Application entry, calls `cc_switch_lib::run()`
- `lib.rs` - Tauri app builder, command registration, plugin setup

## Core Modules

| Module | Purpose |
|--------|---------|
| `database.rs` | SQLite connection and migrations |
| `error.rs` | `AppError` type for frontend serialization |
| `provider.rs` | Provider data structures |
| `settings.rs` | App settings struct and defaults |
| `mcp.rs` | MCP server management |
| `tray.rs` | System tray menu and actions |
| `state.rs` | `AppState` shared across commands |

## AI Assistant Config Handlers

| Files | Assistant |
|-------|-----------|
| `claude_*.rs` | Claude Code |
| `codex_*.rs` | Codex CLI |
| `gemini_*.rs` | Gemini CLI |
| `opencode_*.rs` | OpenCode |

Each has: `*_config.rs` (read/write), `*_mcp.rs` (MCP), `*_prompts.rs` (prompts)

## Command Pattern

```rust
// src/commands/*.rs
#[tauri::command]
pub async fn get_providers(
    state: State<'_, AppState>
) -> Result<Vec<Provider>, AppError> {
    let service = state.provider_service.lock().await;
    service.get_all()
}

// Register in lib.rs
.invoke_handler(tauri::generate_handler![
    commands::provider_commands::get_providers,
    // ...
])
```

## Error Handling

```rust
// src/error.rs
#[derive(Debug, Serialize)]
pub struct AppError {
    pub message: String,
    pub code: Option<String>,
}

// Use thiserror for conversions
impl From<rusqlite::Error> for AppError { ... }
```

## State Management

```rust
// src/state.rs
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
    pub provider_service: Arc<Mutex<ProviderService>>,
    pub proxy_service: Arc<Mutex<ProxyService>>,
    // ...
}
```

## Database

- **Engine**: SQLite via `rusqlite`
- **Location**: Tauri app data directory
- **Migrations**: Applied in `database.rs` on startup

## Async Runtime

Uses **Tokio** for async operations:
```rust
#[tauri::command]
pub async fn start_proxy(...) -> Result<(), AppError> {
    tokio::spawn(async move { ... });
}
```

## Plugins Used

- `tauri-plugin-shell` - Shell command execution
- `tauri-plugin-dialog` - Native dialogs
- `tauri-plugin-fs` - File system access
- `tauri-plugin-opener` - Open URLs/files
- `tauri-plugin-deep-link` - Deep link handling
- `tauri-plugin-autostart` - Launch on startup
- `tauri-plugin-single-instance` - Single instance lock

## When Making Changes

1. **New command**: Add to `commands/`, register in `lib.rs` `invoke_handler`
2. **New service**: Add to `services/`, inject via `AppState`
3. **Database change**: Add migration in `database.rs`
4. **New plugin**: Add to `Cargo.toml`, configure in `lib.rs`
5. **Error handling**: Use `AppError` for frontend-visible errors

## Building

```bash
cargo build           # Debug build
cargo build --release # Release build
cargo test            # Run tests
cargo clippy          # Lint
```
