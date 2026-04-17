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

## Hydra Orchestration Toolkit

Hydra is a Lead-driven orchestration toolkit. You (the Lead) make strategic
decisions at decision points; Hydra handles operational management.
`result.json` is the only completion evidence.

Why this design (vs. other coding-agent products):
- **SWF decider pattern, specialized for LLM deciders.** Hydra is the AWS SWF / Cadence / Temporal decider pattern. `hydra watch` is `PollForDecisionTask`; the Lead is the decider; `lead_terminal_id` enforces single-decider semantics.
- **Parallel-first, not bolted on.** `dispatch` + worktree + `merge` are first-class. Lead sequences nodes manually and passes context explicitly via `--context-ref`. Other products treat parallelism as open research; Hydra makes it the default.
- **Typed result contract.** Workers publish a schema-validated `result.json` (`outcome: completed | stuck | error`, optional `stuck_reason: needs_clarification | needs_credentials | needs_context | blocked_technical`). Other products return free-text final messages and require downstream parsing.
- **Lead intervention points.** `hydra reset --feedback` lets the Lead actually intervene at decision points instead of being block-and-join. A stale or wrong run is one `reset` away.

Core rules:
- Root cause first. Fix the implementation problem before changing tests.
- Do not hack tests, fixtures, or mocks to force a green result.
- Do not add silent fallbacks or swallowed errors.
- An assignment run is only complete when `result.json` exists and passes schema validation.

Workflow patterns:
1. Do the task directly when it is simple, local, or clearly faster without workflow overhead.
2. Use Hydra for ambiguous, risky, parallel, or multi-step work:
   ```
   hydra init --intent "<task>" --repo .
   hydra dispatch --workbench W --dispatch <id> --role <role> --intent "<desc>" --repo .
   hydra watch --workbench W --repo .
   # → DecisionPoint returned, decide next step
   hydra complete --workbench W --repo .
   ```
3. Use a direct isolated worker when only a separate worker is needed:
   `hydra spawn --task "<specific task>" --repo . [--worktree .]`

Agent launch rule:
- When dispatching Claude/Codex through TermCanvas CLI, start a fresh agent terminal with `termcanvas terminal create --prompt "..."`
- Do not use `termcanvas terminal input` for task dispatch; it is not a supported automation path

Workflow control:
- After dispatching, always call `hydra watch`. It returns at decision points.
1. Watch until decision point: `hydra watch --workbench <workbenchId> --repo .`
2. Inspect structured state: `hydra status --workbench <workbenchId> --repo .`
3. Reset a dispatch for rework: `hydra reset --workbench W --dispatch N --feedback "..." --repo .`
4. Approve a dispatch's output: `hydra approve --workbench W --dispatch N --repo .`
5. Merge parallel branches: `hydra merge --workbench W --dispatches A,B --repo .`
6. View event log: `hydra ledger --workbench <workbenchId> --repo .`
7. Clean up: `hydra cleanup --workbench <workbenchId> --repo .`

Telemetry polling:
1. Treat `hydra watch` as the main polling loop; do not infer progress from terminal prose alone.
2. Before deciding wait / retry / takeover, query:
   - `termcanvas telemetry get --workbench <workbenchId> --repo .`
   - `termcanvas telemetry get --terminal <terminalId>`
   - `termcanvas telemetry events --terminal <terminalId> --limit 20`
3. Trust `derived_status` and `task_status` as the primary decision signals.

`result.json` must contain (slim, schema_version `hydra/result/v0.1`):
- `schema_version`, `workbench_id`, `assignment_id`, `run_id` (passthrough IDs)
- `outcome` (completed/stuck/error — Hydra routes on this)
- `report_file` (path to a `report.md` written alongside `result.json`)

All human-readable content (summary, outputs, evidence, reflection) lives in
`report.md`. Hydra rejects any extra fields in `result.json`. Write `report.md`
first, then publish `result.json` atomically as the final artifact of the run.

When NOT to use: simple fixes, high-certainty tasks, or work that is faster to do directly in the current agent.
