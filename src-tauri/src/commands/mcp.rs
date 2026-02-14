#![allow(non_snake_case)]

use indexmap::IndexMap;
use std::collections::HashMap;
use serde::Serialize;
use tauri::State;

use crate::app_config::{AppType, McpApps, McpServer};
use crate::claude_mcp;
use crate::services::McpService;
use crate::store::AppState;

/// 获取 Claude MCP 状态
#[tauri::command]
pub async fn get_claude_mcp_status() -> Result<claude_mcp::McpStatus, String> {
    claude_mcp::get_mcp_status().map_err(|e| e.to_string())
}

/// 读取 mcp.json 文本内容
#[tauri::command]
pub async fn read_claude_mcp_config() -> Result<Option<String>, String> {
    claude_mcp::read_mcp_json().map_err(|e| e.to_string())
}

/// 新增或更新一个 MCP 服务器条目
#[tauri::command]
pub async fn upsert_claude_mcp_server(id: String, spec: serde_json::Value) -> Result<bool, String> {
    claude_mcp::upsert_mcp_server(&id, spec).map_err(|e| e.to_string())
}

/// 删除一个 MCP 服务器条目
#[tauri::command]
pub async fn delete_claude_mcp_server(id: String) -> Result<bool, String> {
    claude_mcp::delete_mcp_server(&id).map_err(|e| e.to_string())
}

/// 校验命令是否在 PATH 中可用（不执行）
#[tauri::command]
pub async fn validate_mcp_command(cmd: String) -> Result<bool, String> {
    claude_mcp::validate_command_in_path(&cmd).map_err(|e| e.to_string())
}

#[derive(Serialize)]
pub struct McpConfigResponse {
    pub config_path: String,
    pub servers: HashMap<String, serde_json::Value>,
}

/// 获取 MCP 配置（来自 ~/.cc-switch/config.json）
use std::str::FromStr;

#[tauri::command]
#[allow(deprecated)] // 兼容层命令，内部调用已废弃的 Service 方法
pub async fn get_mcp_config(
    state: State<'_, AppState>,
    app: String,
) -> Result<McpConfigResponse, String> {
    let config_path = crate::config::get_app_config_path()
        .to_string_lossy()
        .to_string();
    let app_ty = AppType::from_str(&app).map_err(|e| e.to_string())?;
    let servers = McpService::get_servers(&state, app_ty).map_err(|e| e.to_string())?;
    Ok(McpConfigResponse {
        config_path,
        servers,
    })
}

/// 在 config.json 中新增或更新一个 MCP 服务器定义
/// [已废弃] 该命令仍然使用旧的分应用API，会转换为统一结构
#[tauri::command]
pub async fn upsert_mcp_server_in_config(
    state: State<'_, AppState>,
    app: String,
    id: String,
    spec: serde_json::Value,
    sync_other_side: Option<bool>,
) -> Result<bool, String> {
    use crate::app_config::McpServer;

    let app_ty = AppType::from_str(&app).map_err(|e| e.to_string())?;

    // 读取现有的服务器（如果存在）
    let existing_server = {
        let servers = state.db.get_all_mcp_servers().map_err(|e| e.to_string())?;
        servers.get(&id).cloned()
    };

    // 构建新的统一服务器结构
    let mut new_server = if let Some(mut existing) = existing_server {
        // 更新现有服务器
        existing.server = spec.clone();
        existing.apps.set_enabled_for(&app_ty, true);
        existing
    } else {
        // 创建新服务器
        let mut apps = McpApps::default();
        apps.set_enabled_for(&app_ty, true);

        // 尝试从 spec 中提取 name，否则使用 id
        let name = spec
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or(&id)
            .to_string();

        McpServer {
            id: id.clone(),
            name,
            server: spec,
            apps,
            description: None,
            homepage: None,
            docs: None,
            tags: Vec::new(),
        }
    };

    // 如果 sync_other_side 为 true，也启用其他应用
    if sync_other_side.unwrap_or(false) {
        new_server.apps.claude = true;
        new_server.apps.codex = true;
        new_server.apps.gemini = true;
        new_server.apps.opencode = true;
    }

    McpService::upsert_server(&state, new_server)
        .map(|_| true)
        .map_err(|e| e.to_string())
}

/// 在 config.json 中删除一个 MCP 服务器定义
#[tauri::command]
pub async fn delete_mcp_server_in_config(
    state: State<'_, AppState>,
    _app: String, // 参数保留用于向后兼容，但在统一结构中不再需要
    id: String,
) -> Result<bool, String> {
    McpService::delete_server(&state, &id).map_err(|e| e.to_string())
}

/// 设置启用状态并同步到客户端配置
#[tauri::command]
#[allow(deprecated)] // 兼容层命令，内部调用已废弃的 Service 方法
pub async fn set_mcp_enabled(
    state: State<'_, AppState>,
    app: String,
    id: String,
    enabled: bool,
) -> Result<bool, String> {
    let app_ty = AppType::from_str(&app).map_err(|e| e.to_string())?;
    McpService::set_enabled(&state, app_ty, &id, enabled).map_err(|e| e.to_string())
}

// ============================================================================
// v3.7.0 新增：统一 MCP 管理命令
// ============================================================================

/// 获取所有 MCP 服务器（统一结构）
#[tauri::command]
pub async fn get_mcp_servers(
    state: State<'_, AppState>,
) -> Result<IndexMap<String, McpServer>, String> {
    McpService::get_all_servers(&state).map_err(|e| e.to_string())
}

/// 添加或更新 MCP 服务器
#[tauri::command]
pub async fn upsert_mcp_server(
    state: State<'_, AppState>,
    server: McpServer,
) -> Result<(), String> {
    McpService::upsert_server(&state, server).map_err(|e| e.to_string())
}

/// 删除 MCP 服务器
#[tauri::command]
pub async fn delete_mcp_server(state: State<'_, AppState>, id: String) -> Result<bool, String> {
    McpService::delete_server(&state, &id).map_err(|e| e.to_string())
}

/// 切换 MCP 服务器在指定应用的启用状态
#[tauri::command]
pub async fn toggle_mcp_app(
    state: State<'_, AppState>,
    server_id: String,
    app: String,
    enabled: bool,
) -> Result<(), String> {
    let app_ty = AppType::from_str(&app).map_err(|e| e.to_string())?;
    McpService::toggle_app(&state, &server_id, app_ty, enabled).map_err(|e| e.to_string())
}

/// 从所有应用导入 MCP 服务器（复用已有的导入逻辑）
#[tauri::command]
pub async fn import_mcp_from_apps(state: State<'_, AppState>) -> Result<usize, String> {
    let mut total = 0;
    total += McpService::import_from_claude(&state).unwrap_or(0);
    total += McpService::import_from_codex(&state).unwrap_or(0);
    total += McpService::import_from_gemini(&state).unwrap_or(0);
    total += McpService::import_from_opencode(&state).unwrap_or(0);
    Ok(total)
}

/// MCP 连通性检测结果
#[derive(Debug, Serialize)]
pub struct McpConnectivityResult {
    pub ok: bool,
    pub message: String,
}

/// 测试 MCP 服务器连通性
#[tauri::command]
pub async fn test_mcp_connectivity(
    server: serde_json::Value,
) -> Result<McpConnectivityResult, String> {
    let server_type = server
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("stdio");

    match server_type {
        "stdio" => {
            let command = server
                .get("command")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if command.is_empty() {
                return Ok(McpConnectivityResult {
                    ok: false,
                    message: "No command specified".to_string(),
                });
            }
            // 检查命令是否存在
            let which_cmd = if cfg!(target_os = "windows") {
                "where"
            } else {
                "which"
            };
            match std::process::Command::new(which_cmd).arg(command).output() {
                Ok(output) if output.status.success() => {
                    let path = String::from_utf8_lossy(&output.stdout)
                        .trim()
                        .lines()
                        .next()
                        .unwrap_or("")
                        .to_string();
                    Ok(McpConnectivityResult {
                        ok: true,
                        message: format!("Command found: {}", path),
                    })
                }
                Ok(_) => Ok(McpConnectivityResult {
                    ok: false,
                    message: format!("Command not found: {}", command),
                }),
                Err(e) => Ok(McpConnectivityResult {
                    ok: false,
                    message: format!("Failed to check command: {}", e),
                }),
            }
        }
        "http" | "sse" => {
            let url = server
                .get("url")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if url.is_empty() {
                return Ok(McpConnectivityResult {
                    ok: false,
                    message: "No URL specified".to_string(),
                });
            }
            // 发送 HEAD 请求检测 URL 可达性
            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .map_err(|e| e.to_string())?;

            // 构建请求，附加 headers
            let mut request = client.head(url);
            if let Some(headers_obj) = server.get("headers").and_then(|v| v.as_object()) {
                for (k, v) in headers_obj {
                    if let Some(val) = v.as_str() {
                        if let (Ok(name), Ok(value)) = (
                            reqwest::header::HeaderName::from_bytes(k.as_bytes()),
                            reqwest::header::HeaderValue::from_str(val),
                        ) {
                            request = request.header(name, value);
                        }
                    }
                }
            }

            match request.send().await {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() || status == 405 || status == 426 {
                        // 405 Method Not Allowed (HEAD not supported) 和 426 Upgrade Required (SSE) 也算可达
                        Ok(McpConnectivityResult {
                            ok: true,
                            message: format!("Server reachable (HTTP {})", status.as_u16()),
                        })
                    } else if status == 401 || status == 403 {
                        Ok(McpConnectivityResult {
                            ok: true,
                            message: format!(
                                "Server reachable but auth required (HTTP {})",
                                status.as_u16()
                            ),
                        })
                    } else {
                        Ok(McpConnectivityResult {
                            ok: false,
                            message: format!("Server returned HTTP {}", status.as_u16()),
                        })
                    }
                }
                Err(e) => {
                    let msg = if e.is_timeout() {
                        "Connection timed out (10s)".to_string()
                    } else if e.is_connect() {
                        format!("Connection refused: {}", url)
                    } else {
                        format!("Request failed: {}", e)
                    };
                    Ok(McpConnectivityResult {
                        ok: false,
                        message: msg,
                    })
                }
            }
        }
        _ => Ok(McpConnectivityResult {
            ok: false,
            message: format!("Unknown server type: {}", server_type),
        }),
    }
}

/// 解析 JSON 文件中的 MCP 服务器配置（自动检测格式）
#[derive(Debug, Serialize)]
pub struct ParsedMcpEntry {
    pub name: String,
    pub server: serde_json::Value,
}

#[tauri::command]
pub async fn parse_mcp_json_file(path: String) -> Result<Vec<ParsedMcpEntry>, String> {
    let content = std::fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))?;
    let json: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("Invalid JSON: {}", e))?;

    let obj = json.as_object().ok_or("JSON root must be an object")?;

    // 格式自动检测
    // 1. OpenCode 格式: { mcp: { servers: { name: { type: "local"|"remote", ... } } } }
    if let Some(mcp) = obj.get("mcp").and_then(|v| v.as_object()) {
        if let Some(servers) = mcp.get("servers").and_then(|v| v.as_object()) {
            return Ok(convert_opencode_format(servers));
        }
    }

    // 2. CC-Switch 内部格式: entries 含 server + apps 字段
    if obj.values().any(|v| v.get("server").is_some() && v.get("apps").is_some()) {
        return Ok(obj
            .iter()
            .filter_map(|(name, entry)| {
                entry.get("server").map(|server| ParsedMcpEntry {
                    name: name.clone(),
                    server: server.clone(),
                })
            })
            .collect());
    }

    // 3. Codex 格式: { mcp_servers: { name: { ... } } }
    if let Some(servers) = obj.get("mcp_servers").and_then(|v| v.as_object()) {
        return Ok(convert_codex_format(servers));
    }

    // 4. Claude/Gemini/标准 MCP 格式: { mcpServers: { name: { ... } } }
    if let Some(servers) = obj.get("mcpServers").and_then(|v| v.as_object()) {
        return Ok(convert_standard_format(servers));
    }

    // 5. 裸 map: 顶层对象的值含 command 或 url 字段
    if obj.values().any(|v| v.get("command").is_some() || v.get("url").is_some()) {
        return Ok(convert_standard_format(obj));
    }

    Err("Unrecognized MCP configuration format".to_string())
}

/// 转换标准 MCP 格式 (Claude/Gemini/MCP Router)
fn convert_standard_format(
    servers: &serde_json::Map<String, serde_json::Value>,
) -> Vec<ParsedMcpEntry> {
    servers
        .iter()
        .map(|(name, spec)| {
            let mut server = spec.clone();
            // 确保有 type 字段
            if server.get("type").is_none() {
                let obj = server.as_object_mut().unwrap();
                if obj.contains_key("command") {
                    obj.insert("type".to_string(), serde_json::json!("stdio"));
                } else if obj.contains_key("url") {
                    obj.insert("type".to_string(), serde_json::json!("http"));
                }
            }
            ParsedMcpEntry {
                name: name.clone(),
                server,
            }
        })
        .collect()
}

/// 转换 OpenCode 格式 (local → stdio, remote → http/sse)
fn convert_opencode_format(
    servers: &serde_json::Map<String, serde_json::Value>,
) -> Vec<ParsedMcpEntry> {
    servers
        .iter()
        .map(|(name, spec)| {
            let oc_type = spec.get("type").and_then(|v| v.as_str()).unwrap_or("local");
            let mut server = serde_json::Map::new();

            match oc_type {
                "local" => {
                    server.insert("type".to_string(), serde_json::json!("stdio"));
                    // OpenCode 的 command 是 string[] (合并了 cmd + args)
                    if let Some(cmd_arr) = spec.get("command").and_then(|v| v.as_array()) {
                        if let Some(first) = cmd_arr.first().and_then(|v| v.as_str()) {
                            server.insert("command".to_string(), serde_json::json!(first));
                        }
                        if cmd_arr.len() > 1 {
                            let args: Vec<&serde_json::Value> = cmd_arr[1..].iter().collect();
                            server.insert("args".to_string(), serde_json::json!(args));
                        }
                    }
                    if let Some(env) = spec.get("environment") {
                        server.insert("env".to_string(), env.clone());
                    }
                }
                "remote" => {
                    server.insert("type".to_string(), serde_json::json!("http"));
                    if let Some(url) = spec.get("url") {
                        server.insert("url".to_string(), url.clone());
                    }
                    if let Some(headers) = spec.get("headers") {
                        server.insert("headers".to_string(), headers.clone());
                    }
                }
                other => {
                    server.insert("type".to_string(), serde_json::json!(other));
                }
            }

            ParsedMcpEntry {
                name: name.clone(),
                server: serde_json::Value::Object(server),
            }
        })
        .collect()
}

/// 转换 Codex 格式 (http_headers → headers)
fn convert_codex_format(
    servers: &serde_json::Map<String, serde_json::Value>,
) -> Vec<ParsedMcpEntry> {
    servers
        .iter()
        .map(|(name, spec)| {
            let mut server = spec.clone();
            // Codex 使用 http_headers 而非 headers
            if let Some(obj) = server.as_object_mut() {
                if let Some(http_headers) = obj.remove("http_headers") {
                    obj.insert("headers".to_string(), http_headers);
                }
                // 确保有 type 字段
                if !obj.contains_key("type") {
                    if obj.contains_key("command") {
                        obj.insert("type".to_string(), serde_json::json!("stdio"));
                    } else if obj.contains_key("url") {
                        obj.insert("type".to_string(), serde_json::json!("sse"));
                    }
                }
            }
            ParsedMcpEntry {
                name: name.clone(),
                server,
            }
        })
        .collect()
}
