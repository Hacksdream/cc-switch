# Lib (Utilities & API Layer)

## Overview

Core utilities, API wrappers for Tauri backend communication, TanStack Query setup, and Zod validation schemas.

## Directory Structure

| Directory | Purpose |
|-----------|---------|
| `api/` | Tauri `invoke()` wrappers for backend commands |
| `query/` | TanStack Query configuration (queries, mutations) |
| `schemas/` | Zod validation schemas |
| `errors/` | Error handling utilities |
| `utils/` | General utility functions |

## Root Files

| File | Purpose |
|------|---------|
| `platform.ts` | Platform detection (macOS, Windows, Linux) |
| `updater.ts` | App update checking and installation |
| `utils.ts` | Common utility functions |

## API Layer (`api/`)

Each file wraps Tauri commands for a specific domain:

| File | Domain | Key Functions |
|------|--------|---------------|
| `providers.ts` | Provider CRUD | `getAll`, `add`, `update`, `delete`, `switch` |
| `settings.ts` | App settings | `get`, `update` |
| `mcp.ts` | MCP servers | `list`, `add`, `remove`, `toggle` |
| `prompts.ts` | Prompts | `list`, `create`, `update`, `delete` |
| `skills.ts` | Skills | `list`, `install`, `uninstall` |
| `sessions.ts` | Sessions | `list`, `get`, `delete` |
| `usage.ts` | Usage stats | `getSummary`, `getLogs` |
| `proxy.ts` | Proxy server | `start`, `stop`, `getStatus` |
| `config.ts` | Config import/export | `export`, `import` |
| `vscode.ts` | VS Code integration | `openInVSCode` |
| `types.ts` | Shared types | `AppId` enum |

### Pattern
```typescript
import { invoke } from "@tauri-apps/api/core";

export const providersApi = {
  async getAll(appId: AppId): Promise<Record<string, Provider>> {
    return await invoke("get_providers", { app: appId });
  },
  // ... other methods
};
```

### Event Listening
```typescript
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

async onSwitched(handler: (event: ProviderSwitchEvent) => void): Promise<UnlistenFn> {
  return await listen("provider-switched", (event) => {
    handler(event.payload as ProviderSwitchEvent);
  });
}
```

## Query Layer (`query/`)

TanStack Query configuration for server state management:

| File | Purpose |
|------|---------|
| `queries.ts` | Query key factories and query options |
| `mutations.ts` | Mutation functions with cache invalidation |
| `queryClient.ts` | QueryClient configuration |

### Query Pattern
```typescript
import { useQuery } from "@tanstack/react-query";
import { providersApi } from "@/lib/api";

export const providerQueries = {
  all: (appId: AppId) => ({
    queryKey: ["providers", appId],
    queryFn: () => providersApi.getAll(appId),
  }),
};

// Usage in component
const { data: providers } = useQuery(providerQueries.all("claude"));
```

## Schemas (`schemas/`)

Zod schemas for form validation and type inference:

```typescript
import { z } from "zod";

export const providerSchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(1),
  apiKey: z.string().min(1),
  baseUrl: z.string().url().optional(),
});

export type Provider = z.infer<typeof providerSchema>;
```

## When Making Changes

1. **New backend command**: Add wrapper in `api/{domain}.ts`
2. **New query**: Add to `query/queries.ts` with proper cache key
3. **New mutation**: Add to `query/mutations.ts` with invalidation
4. **Form validation**: Add Zod schema in `schemas/`
5. **Platform-specific code**: Use `platform.ts` detection
