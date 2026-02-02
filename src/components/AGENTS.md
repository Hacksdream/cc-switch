# Components

## Overview

React components organized by feature. Uses shadcn/ui primitives, Framer Motion for animations, and follows a feature-based directory structure.

## Directory Structure

| Directory | Purpose |
|-----------|---------|
| `agents/` | Agent configuration components (per-assistant) |
| `common/` | Shared/reusable components |
| `deeplink/` | Deep link handling and import dialogs |
| `env/` | Environment variable management |
| `icons/` | Custom icon components |
| `mcp/` | MCP server configuration UI |
| `prompts/` | Prompt management (list, editor) |
| `providers/` | Provider cards, dialogs, forms |
| `proxy/` | Proxy configuration and status |
| `sessions/` | Session management UI |
| `settings/` | App settings panels |
| `skills/` | Skill management (browser, editor) |
| `ui/` | shadcn/ui primitives (Button, Dialog, etc.) |
| `universal/` | Universal provider configuration |
| `usage/` | Usage tracking and statistics |

## Root Components

| File | Purpose |
|------|---------|
| `AppSwitcher.tsx` | Main app/tab navigation |
| `BrandIcons.tsx` | AI assistant brand icons |
| `ColorPicker.tsx` | Color selection component |
| `ConfirmDialog.tsx` | Confirmation modal |
| `DeepLinkImportDialog.tsx` | Import from deep link |
| `IconPicker.tsx` | Icon selection grid |
| `JsonEditor.tsx` | JSON editor with validation |
| `MarkdownEditor.tsx` | Markdown editing component |
| `ProviderIcon.tsx` | Provider icon display |
| `UpdateBadge.tsx` | Update notification badge |
| `UsageFooter.tsx` | Usage stats in footer |
| `UsageScriptModal.tsx` | Usage script configuration |
| `mode-toggle.tsx` | Light/dark mode toggle |
| `theme-provider.tsx` | Theme context provider |

## Patterns

### Feature Component Structure
```
{feature}/
├── {Feature}Page.tsx       # Main page component
├── {Feature}List.tsx       # List/grid view
├── {Feature}Card.tsx       # Individual item card
├── {Feature}Dialog.tsx     # Add/edit dialog
└── {Feature}Form.tsx       # Form for CRUD
```

### Using shadcn/ui
```typescript
// Import from local ui/ directory
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { Card, CardHeader, CardContent } from "@/components/ui/card";
```

### Framer Motion Animations
```typescript
import { motion, AnimatePresence } from "framer-motion";

<AnimatePresence>
  {isVisible && (
    <motion.div
      initial={{ opacity: 0, y: 10 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -10 }}
    >
      {children}
    </motion.div>
  )}
</AnimatePresence>
```

### i18n Usage
```typescript
import { useTranslation } from "react-i18next";

const { t } = useTranslation();
return <Button>{t("common.save")}</Button>;
```

## When Adding Components

1. **New feature**: Create `{feature}/` directory with page, list, card, dialog
2. **Reusable component**: Add to `common/` directory
3. **UI primitive**: Use shadcn/ui CLI: `npx shadcn@latest add {component}`
4. **With state**: Use TanStack Query for server state, useState for UI state
5. **With animations**: Use Framer Motion `motion` components
