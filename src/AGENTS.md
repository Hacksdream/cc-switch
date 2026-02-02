# Frontend (React + TypeScript)

## Overview

React 18 frontend built with Vite, using TanStack Query for server state, shadcn/ui for components, and Framer Motion for animations.

## Directory Structure

| Directory | Purpose | AGENTS.md |
|-----------|---------|-----------|
| `components/` | React components organized by feature | [components/AGENTS.md](components/AGENTS.md) |
| `hooks/` | Custom React hooks | — |
| `lib/` | Utilities, API wrappers, queries | [lib/AGENTS.md](lib/AGENTS.md) |
| `config/` | Provider presets and app configuration | — |
| `i18n/` | Internationalization (en, zh, ja) | — |
| `types/` | TypeScript type definitions | — |

## Entry Points

- `main.tsx` - App bootstrap, React root, providers setup
- `App.tsx` - Main component with routing and layout

## Key Patterns

### State Management
```typescript
// Server state: TanStack Query
const { data: providers } = useQuery(providerQueries.list());

// UI state: React useState/useReducer
const [isOpen, setIsOpen] = useState(false);
```

### Tauri Communication
```typescript
// All backend calls go through src/lib/api/*.ts
import { getProviders } from "@/lib/api/providers";
const providers = await getProviders();
```

### Component Structure
```typescript
// Feature components in src/components/{feature}/
// UI primitives in src/components/ui/
import { Button } from "@/components/ui/button";
import { ProviderCard } from "@/components/providers/ProviderCard";
```

## Hooks (`hooks/`)

| Hook | Purpose |
|------|---------|
| `useProviderActions` | Provider CRUD operations |
| `useProxyStatus` | Proxy server status |
| `useMcp` | MCP server management |
| `useSkills` | Skill management |
| `useSettings` | App settings |
| `useClaudeStatus` | Claude installation status |
| `useCodexStatus` | Codex installation status |
| `useGeminiStatus` | Gemini installation status |
| `useOpencodeStatus` | OpenCode installation status |

## Configuration (`config/`)

Provider presets for each AI assistant:
- `claudeProviderPresets.ts`
- `codexProviderPresets.ts`
- `geminiProviderPresets.ts`
- `opencodeProviderPresets.ts`

## i18n (`i18n/`)

Uses i18next with namespaced JSON files:
- `locales/en.json` - English
- `locales/zh.json` - Chinese
- `locales/ja.json` - Japanese

**Adding translations**: Update ALL locale files when adding new strings.

## Styling

- **Tailwind CSS** for utility classes
- **CSS Variables** for theming (defined in `index.css`)
- **shadcn/ui** components (Radix-based, in `components/ui/`)

## Path Aliases

```typescript
// tsconfig.json defines @ -> src/
import { Button } from "@/components/ui/button";
import { useSettings } from "@/hooks/useSettings";
```

## When Making Changes

1. **New feature**: Create `components/{feature}/` directory with components
2. **New hook**: Add to `hooks/` with `use` prefix
3. **New API call**: Add wrapper in `lib/api/`, query in `lib/query/`
4. **New UI component**: Use shadcn/ui CLI or add to `components/ui/`
5. **Translations**: Update all 3 locale files
