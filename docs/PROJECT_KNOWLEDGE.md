# CC-Switch 项目知识库

> 本文档由 AI 助手自动生成，用于项目上下文理解和快速上手。

## 1. 项目概览

**名称**: CC-Switch (Claude Code Switch)  
**版本**: 3.10.3  
**类型**: Tauri 2 桌面应用  
**用途**: AI 编程助手配置管理器（支持 Claude Code, Codex CLI, Gemini CLI, OpenCode）

## 2. 技术栈

### 前端

| 技术              | 版本 | 用途              |
| ----------------- | ---- | ----------------- |
| React             | 18.2 | UI 框架           |
| Vite              | 7.3  | 构建工具          |
| TypeScript        | 5.3  | 类型系统          |
| TanStack Query    | 5.90 | 服务端状态管理    |
| shadcn/ui + Radix | -    | UI 组件库         |
| Framer Motion     | 12   | 动画              |
| i18next           | 25.5 | 国际化 (en/zh/ja) |
| Tailwind          | 3.4  | CSS 框架          |
| Zod               | 4.1  | 表单验证          |
| react-hook-form   | 7.65 | 表单处理          |

### 后端

| 技术          | 版本   | 用途             |
| ------------- | ------ | ---------------- |
| Tauri         | 2.8.2  | 桌面应用框架     |
| Rust          | 1.85.0 | 后端语言         |
| rusqlite      | -      | SQLite 数据库    |
| Tokio         | 1.x    | 异步运行时       |
| reqwest/hyper | 1.0    | HTTP 客户端/代理 |
| serde         | -      | 序列化           |

## 3. 常用命令

```bash
# 开发
pnpm tauri dev          # 启动开发模式（前后端热重载）
pnpm dev:renderer       # 仅前端开发

# 构建
pnpm tauri build        # 生产构建
pnpm build:renderer     # 仅前端构建

# 代码质量
pnpm typecheck          # TypeScript 类型检查
pnpm lint               # ESLint 检查
pnpm format             # Prettier 格式化

# 测试
pnpm test:unit          # 前端单元测试 (Vitest)
cd src-tauri && cargo test  # Rust 测试
```

## 4. 项目结构

```
cc-switch/
├── src/                        # 前端源码 (React)
│   ├── components/             # React 组件
│   │   ├── mcp/               # MCP 服务器管理
│   │   ├── prompts/           # 提示词管理
│   │   ├── providers/         # Provider 管理
│   │   ├── skills/            # Skills 管理
│   │   ├── settings/          # 设置页面
│   │   ├── ui/                # shadcn/ui 基础组件
│   │   └── ...
│   ├── hooks/                  # 自定义 React Hooks
│   ├── lib/
│   │   ├── api/               # Tauri invoke 封装
│   │   ├── query/             # TanStack Query hooks
│   │   └── schemas/           # Zod 验证 schemas
│   ├── i18n/locales/          # 国际化文件 (en.json, zh.json, ja.json)
│   └── App.tsx                # 主入口 (1093 行)
│
├── src-tauri/                  # 后端源码 (Rust)
│   ├── src/
│   │   ├── commands/          # Tauri 命令处理器
│   │   ├── database/          # SQLite 操作 + DAO
│   │   ├── mcp/               # MCP 配置 (per AI assistant)
│   │   ├── deeplink/          # URL scheme 处理
│   │   ├── claude_*.rs        # Claude 特定配置
│   │   ├── codex_*.rs         # Codex 特定配置
│   │   ├── gemini_*.rs        # Gemini 特定配置
│   │   ├── opencode_*.rs      # OpenCode 特定配置
│   │   ├── lib.rs             # 主入口 (55KB)
│   │   └── error.rs           # 错误处理
│   └── Cargo.toml
│
├── .github/workflows/          # CI/CD
│   ├── build.yml              # 构建 (push to main → draft release)
│   └── release.yml            # 发布 (tag v* → prerelease)
│
└── scripts/                    # 构建脚本
```

## 5. 开发规范

### 文件命名

| 上下文          | 规范       | 示例                      |
| --------------- | ---------- | ------------------------- |
| React 组件      | PascalCase | `ProviderCard.tsx`        |
| TypeScript 文件 | kebab-case | `use-provider-actions.ts` |
| Rust 文件       | snake_case | `provider_commands.rs`    |

### 添加新功能的步骤

1. **组件**: `src/components/{feature}/`
2. **API 封装**: `src/lib/api/{feature}.ts`
3. **Rust 命令**: `src-tauri/src/commands/{feature}.rs`
4. **注册命令**: `src-tauri/src/lib.rs` 中添加
5. **国际化**: 更新 **所有 3 个** locale 文件 (en/zh/ja)

### 前后端通信

```typescript
// 前端调用
import { invoke } from "@tauri-apps/api/core";
const result = await invoke<Provider[]>("get_providers");

// 后端处理
#[tauri::command]
pub async fn get_providers(state: State<'_, AppState>) -> Result<Vec<Provider>, AppError> { ... }
```

### 错误处理

- **前端**: try/catch + sonner toast
- **后端**: `AppError` 类型，支持双语错误 (`AppError::localized`)

## 6. 仓库信息

- **Fork**: `Hacksdream/cc-switch` (用户仓库)
- **Upstream**: `farion1231/cc-switch` (原始仓库)
- **主要贡献者**: Jason (873), YoVinchen (430), farion1231 (100)
- **分支策略**: `main` + `feat/*` 功能分支

## 7. 支持的 AI 助手

| 助手        | 后端模块             | 配置格式                               |
| ----------- | -------------------- | -------------------------------------- |
| Claude Code | `claude_*.rs`        | JSON (claude_mcp.rs, claude_plugin.rs) |
| Codex CLI   | `codex_config.rs`    | 自定义格式                             |
| Gemini CLI  | `gemini_*.rs`        | JSON                                   |
| OpenCode    | `opencode_config.rs` | TOML                                   |

## 8. 核心功能模块

1. **Provider 管理** - API 提供商配置
2. **MCP 服务器** - Model Context Protocol 服务配置
3. **Prompts 管理** - 系统提示词管理
4. **Skills 管理** - 技能定义管理
5. **Proxy & Failover** - HTTP 代理和故障转移
6. **Sessions** - 会话追踪
7. **Usage** - API 使用量统计
8. **Deep Linking** - `ccswitch://` URL scheme 导入

## 9. 数据库架构

- **存储**: SQLite via rusqlite
- **Schema 版本**: 5 (修改表结构时需要递增)
- **备份保留**: 10 个备份
- **DAO 模式**: `src-tauri/src/database/dao/` 目录下按功能划分

### 主要 DAO 文件

| 文件           | 功能              |
| -------------- | ----------------- |
| `providers.rs` | Provider CRUD     |
| `proxy.rs`     | 代理/故障转移队列 |
| `skills.rs`    | Skills 管理       |
| `settings.rs`  | 设置存储          |
| `mcp.rs`       | MCP 服务器        |
| `prompts.rs`   | 提示词            |

## 10. CI/CD

### build.yml (推送到 main)

- **触发**: Push to main (排除 docs/md/assets)
- **平台**: Windows 2022, Ubuntu 22.04 (x64/arm64), macOS 14
- **输出**: Draft prerelease (`dev-{version}-{sha}`)

### release.yml (推送 tag)

- **触发**: Push tag `v*`
- **输出**: Prerelease (非 draft)

## 11. 关键类型

```typescript
// AI 助手标识符
export type AppId = "claude" | "codex" | "gemini" | "opencode";
```

---

_最后更新: 2026-02-09_
