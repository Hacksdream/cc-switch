# Commands (Tauri Command Handlers)

## Overview

Tauri commands are the bridge between frontend and backend. Each command is exposed to the frontend via `invoke()` and handles request/response serialization automatically.

## Module Index

| Module | Purpose |
|--------|---------|
| `config.rs` | Config import/export |
| `deeplink.rs` | Deep link URL handling |
| `env.rs` | Environment variable management |
| `failover.rs` | Failover configuration |
| `global_proxy.rs` | Global proxy settings |
| `import_export.rs` | Bulk import/export |
| `mcp.rs` | MCP server commands |
| `misc.rs` | Miscellaneous commands |
| `plugin.rs` | Plugin management |
| `prompt.rs` | Prompt CRUD commands |
| `provider.rs` | Provider management |
| `proxy.rs` | Proxy server control |
| `settings.rs` | Settings commands |
| `session_manager.rs` | Session management |
| `skill.rs` | Skill commands |
| `stream_check.rs` | Streaming API verification |
| `usage.rs` | Usage tracking queries |

## Command Pattern

```rust
use tauri::State;
use crate::{error::AppError, state::AppState};

#[tauri::command]
pub async fn get_providers(
    app: String,
    state: State<'_, AppState>,
) -> Result<HashMap<String, Provider>, AppError> {
    let service = state.provider_service.lock().await;
    service.get_all(&app).await
}
```

### Key Elements
1. `#[tauri::command]` - Macro to expose function to frontend
2. `State<'_, AppState>` - Dependency injection of app state
3. `Result<T, AppError>` - All commands return Result with custom error type
4. `async` - Most commands are async for I/O operations

## Registration

Commands must be registered in `src-tauri/src/lib.rs`:

```rust
.invoke_handler(tauri::generate_handler![
    commands::get_providers,
    commands::add_provider,
    commands::switch_provider,
    // ... all commands
])
```

## Error Handling

```rust
use crate::error::AppError;

#[tauri::command]
pub async fn risky_operation(state: State<'_, AppState>) -> Result<(), AppError> {
    // Errors are automatically serialized to frontend
    let result = do_something().map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(result)
}
```

## Common Patterns

### CRUD Commands
```rust
// get_* - Read operations
#[tauri::command]
pub async fn get_provider(id: String, state: State<'_, AppState>) -> Result<Provider, AppError>

// add_* - Create operations
#[tauri::command]
pub async fn add_provider(provider: Provider, state: State<'_, AppState>) -> Result<bool, AppError>

// update_* - Update operations
#[tauri::command]
pub async fn update_provider(provider: Provider, state: State<'_, AppState>) -> Result<bool, AppError>

// delete_* - Delete operations
#[tauri::command]
pub async fn delete_provider(id: String, state: State<'_, AppState>) -> Result<bool, AppError>
```

### Event Emission
```rust
use tauri::Emitter;

#[tauri::command]
pub async fn switch_provider(id: String, app_handle: tauri::AppHandle, state: State<'_, AppState>) -> Result<bool, AppError> {
    // ... switch logic
    app_handle.emit("provider-switched", ProviderSwitchEvent { app_type, provider_id })?;
    Ok(true)
}
```

## When Adding Commands

1. Create function with `#[tauri::command]` in appropriate module
2. Add `pub use module::*;` in `mod.rs` if new module
3. Register in `lib.rs` `invoke_handler`
4. Add frontend wrapper in `src/lib/api/{domain}.ts`
5. Test with `pnpm tauri dev`
