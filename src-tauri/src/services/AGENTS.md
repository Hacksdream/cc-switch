# Services (Business Logic Layer)

## Overview

Services encapsulate business logic, database operations, and external integrations. They are injected into commands via `AppState`.

## Module Index

| Module | Service | Purpose |
|--------|---------|---------|
| `config.rs` | `ConfigService` | App configuration management |
| `env_checker.rs` | — | Environment validation utilities |
| `env_manager.rs` | — | Environment variable management |
| `mcp.rs` | `McpService` | MCP server lifecycle management |
| `prompt.rs` | `PromptService` | Prompt CRUD and storage |
| `provider.rs` | `ProviderService` | Provider management and switching |
| `proxy.rs` | `ProxyService` | Proxy server lifecycle |
| `skill.rs` | `SkillService` | Skill discovery and management |
| `speedtest.rs` | `SpeedtestService` | API latency testing |
| `stream_check.rs` | — | Stream API verification |
| `usage_stats.rs` | — | Usage statistics aggregation |

## Service Pattern

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ProviderService {
    db: Arc<Mutex<Database>>,
    config_path: PathBuf,
}

impl ProviderService {
    pub fn new(db: Arc<Mutex<Database>>, config_path: PathBuf) -> Self {
        Self { db, config_path }
    }

    pub async fn get_all(&self, app_id: &str) -> Result<HashMap<String, Provider>, AppError> {
        let db = self.db.lock().await;
        db.query_providers(app_id).await
    }

    pub async fn switch(&self, id: &str, app_id: &str) -> Result<bool, AppError> {
        // Business logic here
        self.update_live_config(id, app_id).await?;
        self.write_config_file(app_id).await?;
        Ok(true)
    }
}
```

## AppState Integration

Services are stored in `AppState` and accessed via Tauri's dependency injection:

```rust
// state.rs
pub struct AppState {
    pub provider_service: Arc<Mutex<ProviderService>>,
    pub mcp_service: Arc<Mutex<McpService>>,
    pub proxy_service: Arc<Mutex<ProxyService>>,
    // ...
}

// commands/*.rs
#[tauri::command]
pub async fn get_providers(state: State<'_, AppState>) -> Result<...> {
    let service = state.provider_service.lock().await;
    service.get_all("claude").await
}
```

## Key Services

### ProviderService
- Provider CRUD operations
- Live config file management
- Provider switching logic

### McpService
- MCP server configuration
- Server lifecycle (start/stop)
- Configuration file generation

### ProxyService
- HTTP proxy server management
- Failover configuration
- Health monitoring

### SkillService
- Skill discovery from repositories
- Skill installation/uninstallation
- Skill repository management

### SpeedtestService
- API endpoint latency testing
- Provider availability checking

## Async Patterns

```rust
use tokio::sync::Mutex;

// Mutex for thread-safe access
let db = self.db.lock().await;

// Concurrent operations
let (providers, settings) = tokio::join!(
    self.get_providers(),
    self.get_settings()
);

// Timeout
use tokio::time::{timeout, Duration};
let result = timeout(Duration::from_secs(5), async_operation()).await?;
```

## When Adding Services

1. Create struct with dependencies (db, config paths, etc.)
2. Implement `new()` constructor
3. Add async methods for business operations
4. Add to `AppState` in `state.rs`
5. Initialize in `lib.rs` app setup
6. Create commands in `commands/` to expose to frontend
