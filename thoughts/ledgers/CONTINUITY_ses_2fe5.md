---
session: ses_2fe5
updated: 2026-03-18T15:48:58.920Z
---

# Session Summary

## Goal
Identify fork-specific customizations in the CC Switch repository that are most likely to be broken by upstream merges, producing a concise list of critical files/features with exact file paths and preservation checks.

## Constraints & Preferences
- **Tools**: Local codebase search tools only
- **Must Do**: Focus on known custom areas (i18n custom keys, Provider sync button, MCP import/test features, OMO CST write path, build workflow file), include line-level references when possible
- **Must Not Do**: Do not modify files, do not run git push/tag
- **Context**: Prior merges previously dropped i18n keys and dependency jsonc-parser

## Progress
### Done
- [x] Searched codebase for "omo", "cst", "custom", "sync" patterns across JSON and Rust files
- [x] Located jsonc-parser dependency in `Cargo.toml` line 68 (`jsonc-parser = { version = "0.29", features = ["cst", "serde"] }`) and `package.json` line 82 (`"jsonc-parser": "^3.2.1"`)
- [x] Identified i18n custom keys in all three locale files: `provider.syncFromConfig` (lines 149-150), `providerForm.omoHint` (line 699), `providerForm.partnerPromotion.*` (lines 702-718)
- [x] Located Provider sync button in `ProviderList.tsx` lines 445-461
- [x] Located MCP custom components: `UnifiedMcpPanel.tsx`, `McpImportPreviewModal.tsx`, `McpWizardModal.tsx`
- [x] Identified OMO CST write path in `opencode_config.rs`: `parse_cst()` (line 119), `write_config_raw()` (line 112), `serde_value_to_cst()` (line 124), `deep_merge_cst_object()` (line 148), `cst_set_object_property()` (line 176)
- [x] Identified OMO service variants in `omo.rs`: STANDARD and SLIM with filenames `oh-my-opencode.jsonc` and `oh-my-opencode-slim.jsonc` (lines 34-54)
- [x] Located build workflow customizations: private key Base64 handling (lines 103-157), macOS dual packaging (lines 187-220), Linux AppImage + ARM64 (lines 273-306), latest.json assembly (lines 333-427)
- [x] Generated comprehensive preservation checklist with file paths, line numbers, and risk levels

### In Progress
- [ ] None (analysis complete, awaiting user action)

### Blocked
- (none)

## Key Decisions
- **CST-based write path for OMO**: The `opencode_config.rs` uses `jsonc-parser` crate's CST (Concrete Syntax Tree) feature to preserve JSONC comments when writing config files. This is critical for maintaining user formatting/comments in `oh-my-opencode.jsonc` files.
- **Dual-layer jsonc-parser dependency**: Both `Cargo.toml` (Rust, line 68) and `package.json` (TypeScript, line 82) declare jsonc-parser, indicating it was previously dropped in a prior merge attempt.

## Next Steps
1. **Before any upstream merge**: Create a backup branch to preserve current state
2. **i18n verification**: After merging, compare upstream's `en.json`, `zh.json`, `ja.json` against current files to identify dropped keys (`provider.syncFromConfig`, `providerForm.omoHint`, `partnerPromotion.*`)
3. **jsonc-parser check**: Verify `jsonc-parser` dependency exists in both `Cargo.toml` and `package.json` after merge
4. **CST functions audit**: Confirm `parse_cst()`, `write_config_raw()`, `deep_merge_cst_object()`, and `cst_set_object_property()` still exist in `opencode_config.rs`
5. **Workflow comparison**: Compare `.github/workflows/build.yml` and `release.yml` against upstream to ensure private key handling and multi-platform logic is preserved
6. **MCP components**: Verify `UnifiedMcpPanel.tsx`, `McpImportPreviewModal.tsx`, and `McpWizardModal.tsx` are not removed or replaced

## Critical Context
- **Repository**: `/Users/kirito/aigc/open-source-project/cc-switch`
- **Project type**: Tauri 2 desktop app (React + TypeScript frontend, Rust backend)
- **Version**: 3.12.0
- **Prior issues**: Upstream merges previously dropped i18n keys and `jsonc-parser` dependency — this is a known recurring problem
- **High-risk area**: OMO CST write path uses `jsonc-parser::cst` feature which is non-standard; if upstream changes how it writes `opencode.jsonc`, user comments/formatting will be lost

## File Operations
### Read
- `/Users/kirito/aigc/open-source-project/cc-switch/.github/workflows/build.yml`
- `/Users/kirito/aigc/open-source-project/cc-switch/.github/workflows/release.yml`
- `/Users/kirito/aigc/open-source-project/cc-switch/package.json`
- `/Users/kirito/aigc/open-source-project/cc-switch/src-tauri/Cargo.toml`
- `/Users/kirito/aigc/open-source-project/cc-switch/src-tauri/src/opencode_config.rs`
- `/Users/kirito/aigc/open-source-project/cc-switch/src-tauri/src/services/omo.rs`
- `/Users/kirito/aigc/open-source-project/cc-switch/src/components/providers/ProviderList.tsx`
- `/Users/kirito/aigc/open-source-project/cc-switch/src/i18n/locales/en.json`
- `/Users/kirito/aigc/open-source-project/cc-switch/src/i18n/locales/ja.json`
- `/Users/kirito/aigc/open-source-project/cc-switch/src/i18n/locales/zh.json`

### Modified
- (none)
