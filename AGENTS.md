# CC-Switch - AI Coding Assistant Configuration Manager

## Project Overview

CC-Switch is a **Tauri 2 desktop application** for managing configurations of AI coding assistants (Claude Code, Codex CLI, Gemini CLI, OpenCode). It provides a unified interface for provider management, MCP server configuration, prompt/skill management, proxy/failover, and usage tracking.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Desktop Application                       │
├─────────────────────────────────────────────────────────────┤
│  Frontend (React + TypeScript)     │  Backend (Rust + Tauri) │
│  ─────────────────────────────────│──────────────────────────│
│  • React 18 + Vite                │  • Tauri 2.x             │
│  • TanStack Query (server state)  │  • SQLite (rusqlite)     │
│  • shadcn/ui + Radix primitives   │  • Tokio async runtime   │
│  • Framer Motion (animations)     │  • HTTP proxy (hyper)    │
│  • i18next (i18n: en/zh/ja)       │  • System tray           │
│  • Zod (validation)               │  • Deep linking          │
└─────────────────────────────────────────────────────────────┘
```

## Directory Structure

| Directory | Purpose | AGENTS.md |
|-----------|---------|-----------|
| `src/` | React frontend application | [src/AGENTS.md](src/AGENTS.md) |
| `src-tauri/` | Rust backend (Tauri) | [src-tauri/AGENTS.md](src-tauri/AGENTS.md) |
| `scripts/` | Build and utility scripts | — |

## Key Concepts

### Supported AI Assistants
- **Claude Code** (`claude_*.rs`) - Anthropic's coding assistant
- **Codex CLI** (`codex_*.rs`) - OpenAI's CLI tool
- **Gemini CLI** (`gemini_*.rs`) - Google's CLI assistant
- **OpenCode** (`opencode_*.rs`) - Open-source alternative

### Core Features
1. **Provider Management** - Configure API providers (OpenAI, Anthropic, Google, custom)
2. **MCP Servers** - Model Context Protocol server configuration
3. **Prompts & Skills** - Manage system prompts and skill definitions
4. **Proxy & Failover** - HTTP proxy with circuit breaker and failover logic
5. **Sessions** - Track and manage coding sessions
6. **Usage Tracking** - Monitor API usage and costs

## Development Commands

```bash
# Install dependencies
pnpm install

# Development (frontend + backend hot reload)
pnpm tauri dev

# Build for production
pnpm tauri build

# Lint
pnpm lint

# Type check
pnpm typecheck
```

## Communication Pattern

Frontend ↔ Backend communication uses Tauri's `invoke()`:

```typescript
// Frontend: src/lib/api/*.ts
import { invoke } from "@tauri-apps/api/core";
const providers = await invoke<Provider[]>("get_providers");

// Backend: src-tauri/src/commands/*.rs
#[tauri::command]
pub async fn get_providers(state: State<'_, AppState>) -> Result<Vec<Provider>, AppError> { ... }
```

## File Naming Conventions

| Context | Convention | Example |
|---------|------------|---------|
| React components | PascalCase | `ProviderCard.tsx` |
| TypeScript files | kebab-case | `use-provider-actions.ts` |
| Rust files | snake_case | `provider_commands.rs` |
| API wrapper files | kebab-case | `providers.ts` |

## Error Handling

- **Frontend**: Try/catch with toast notifications via `sonner`
- **Backend**: Custom `AppError` type in `src-tauri/src/error.rs`, serialized to frontend

## Testing

```bash
# Frontend tests (Vitest)
pnpm test

# Rust tests
cd src-tauri && cargo test
```

## When Making Changes

1. **Adding a new feature**: Create components in `src/components/{feature}/`, add Tauri commands in `src-tauri/src/commands/`, wire up with `invoke()` in `src/lib/api/`
2. **Adding UI components**: Use shadcn/ui primitives from `src/components/ui/`
3. **Adding translations**: Update all locale files in `src/i18n/locales/`
4. **Modifying backend**: Ensure commands are registered in `src-tauri/src/lib.rs`
