use crate::config::{write_json_file, write_text_file};
use crate::error::AppError;
use crate::provider::OpenCodeProviderConfig;
use crate::settings::get_opencode_override_dir;
use indexmap::IndexMap;
use jsonc_parser::cst::{CstInputValue, CstRootNode};
use jsonc_parser::ParseOptions;
use serde_json::{json, Map, Value};
use std::path::PathBuf;

pub fn get_opencode_dir() -> PathBuf {
    if let Some(override_dir) = get_opencode_override_dir() {
        return override_dir;
    }

    dirs::home_dir()
        .map(|h| h.join(".config").join("opencode"))
        .unwrap_or_else(|| PathBuf::from(".config").join("opencode"))
}

pub fn get_opencode_config_path() -> PathBuf {
    let dir = get_opencode_dir();
    let jsonc_path = dir.join("opencode.jsonc");
    let json_path = dir.join("opencode.json");

    if jsonc_path.exists() {
        jsonc_path
    } else {
        json_path
    }
}

#[allow(dead_code)]
pub fn get_opencode_env_path() -> PathBuf {
    get_opencode_dir().join(".env")
}

fn strip_jsonc_comments(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_string = false;
    let mut escape_next = false;

    while let Some(c) = chars.next() {
        if escape_next {
            result.push(c);
            escape_next = false;
            continue;
        }

        if in_string {
            result.push(c);
            if c == '\\' {
                escape_next = true;
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }

        match c {
            '"' => {
                in_string = true;
                result.push(c);
            }
            '/' => {
                if let Some(&next) = chars.peek() {
                    if next == '/' {
                        chars.next();
                        while let Some(&ch) = chars.peek() {
                            if ch == '\n' {
                                break;
                            }
                            chars.next();
                        }
                    } else if next == '*' {
                        chars.next();
                        while let Some(ch) = chars.next() {
                            if ch == '*' {
                                if let Some(&'/') = chars.peek() {
                                    chars.next();
                                    break;
                                }
                            }
                        }
                    } else {
                        result.push(c);
                    }
                } else {
                    result.push(c);
                }
            }
            _ => result.push(c),
        }
    }

    result
}

// ---------------------------------------------------------------------------
// Raw file I/O (preserves original content for CST round-trip editing)
// ---------------------------------------------------------------------------

fn read_config_raw() -> Result<String, AppError> {
    let path = get_opencode_config_path();
    if !path.exists() {
        return Ok(String::from("{}"));
    }
    std::fs::read_to_string(&path).map_err(|e| AppError::io(&path, e))
}

fn write_config_raw(content: &str) -> Result<(), AppError> {
    let path = get_opencode_config_path();
    write_text_file(&path, content)?;
    log::debug!("OpenCode config written to {path:?}");
    Ok(())
}

fn parse_cst(raw: &str) -> Result<CstRootNode, AppError> {
    CstRootNode::parse(raw, &ParseOptions::default())
        .map_err(|e| AppError::Message(format!("Failed to parse JSONC config: {e:?}")))
}

pub fn serde_value_to_cst(value: &Value) -> CstInputValue {
    match value {
        Value::Null => CstInputValue::Null,
        Value::Bool(b) => CstInputValue::Bool(*b),
        Value::Number(n) => CstInputValue::Number(n.to_string()),
        Value::String(s) => CstInputValue::String(s.clone()),
        Value::Array(arr) => CstInputValue::Array(arr.iter().map(serde_value_to_cst).collect()),
        Value::Object(obj) => CstInputValue::Object(
            obj.iter()
                .map(|(k, v)| (k.clone(), serde_value_to_cst(v)))
                .collect(),
        ),
    }
}

/// Deep-merge a `serde_json::Value::Object` into an existing CST object.
///
/// For each key in `source`:
///   - If both the CST and source values are objects → recurse (preserves comments inside)
///   - Otherwise → shallow `set_value()` (replace leaf values)
///   - New keys → append
///
/// Keys present in CST but absent from `source` are left untouched (preserves
/// unknown fields like `google_auth`).
pub fn deep_merge_cst_object(
    cst_obj: &jsonc_parser::cst::CstObject,
    source: &serde_json::Map<String, Value>,
) {
    for (key, value) in source {
        match value {
            Value::Object(child_map) => {
                // Both sides are objects → recurse to preserve inner comments
                let nested = cst_obj.object_value_or_set(key);
                deep_merge_cst_object(&nested, child_map);
            }
            _ => {
                // Leaf value → shallow replace
                let cst_value = serde_value_to_cst(value);
                if let Some(existing) = cst_obj.get(key) {
                    existing.set_value(cst_value);
                } else {
                    cst_obj.append(key, cst_value);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// CST helpers — set/remove properties while preserving comments & formatting
// ---------------------------------------------------------------------------

fn cst_set_object_property(section: &str, key: &str, value: &Value) -> Result<(), AppError> {
    let raw = read_config_raw()?;
    let root = parse_cst(&raw)?;
    let root_obj = root.object_value_or_set();
    let section_obj = root_obj.object_value_or_set(section);

    let cst_value = serde_value_to_cst(value);

    if let Some(existing) = section_obj.get(key) {
        existing.set_value(cst_value);
    } else {
        section_obj.append(key, cst_value);
    }

    write_config_raw(&root.to_string())
}

fn cst_remove_object_property(section: &str, key: &str) -> Result<(), AppError> {
    let raw = read_config_raw()?;
    let root = parse_cst(&raw)?;
    let root_obj = root.object_value_or_set();

    if let Some(section_obj) = root_obj.object_value(section) {
        if let Some(prop) = section_obj.get(key) {
            prop.remove();
        }
    }

    write_config_raw(&root.to_string())
}

// ---------------------------------------------------------------------------
// Read operations (parse into serde_json::Value — strips comments)
// ---------------------------------------------------------------------------

pub fn read_opencode_config() -> Result<Value, AppError> {
    let path = get_opencode_config_path();

    if !path.exists() {
        return Ok(json!({
            "$schema": "https://opencode.ai/config.json"
        }));
    }

    let content = std::fs::read_to_string(&path).map_err(|e| AppError::io(&path, e))?;
    let stripped = strip_jsonc_comments(&content);
    serde_json::from_str(&stripped).map_err(|e| AppError::json(&path, e))
}

#[allow(dead_code)]
pub fn write_opencode_config(config: &Value) -> Result<(), AppError> {
    let path = get_opencode_config_path();
    write_json_file(&path, config)?;

    log::debug!("OpenCode config written to {path:?}");
    Ok(())
}

// ---------------------------------------------------------------------------
// Provider operations (CST-based — preserves comments)
// ---------------------------------------------------------------------------

pub fn get_providers() -> Result<Map<String, Value>, AppError> {
    let config = read_opencode_config()?;
    Ok(config
        .get("provider")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default())
}

pub fn set_provider(id: &str, config: Value) -> Result<(), AppError> {
    cst_set_object_property("provider", id, &config)
}

pub fn remove_provider(id: &str) -> Result<(), AppError> {
    cst_remove_object_property("provider", id)
}

pub fn get_typed_providers() -> Result<IndexMap<String, OpenCodeProviderConfig>, AppError> {
    let providers = get_providers()?;
    let mut result = IndexMap::new();

    for (id, value) in providers {
        match serde_json::from_value::<OpenCodeProviderConfig>(value.clone()) {
            Ok(config) => {
                result.insert(id, config);
            }
            Err(e) => {
                log::warn!("Failed to parse provider '{id}': {e}");
            }
        }
    }

    Ok(result)
}

pub fn set_typed_provider(id: &str, config: &OpenCodeProviderConfig) -> Result<(), AppError> {
    let value = serde_json::to_value(config).map_err(|e| AppError::JsonSerialize { source: e })?;
    set_provider(id, value)
}

// ---------------------------------------------------------------------------
// MCP operations (CST-based — preserves comments)
// ---------------------------------------------------------------------------

pub fn get_mcp_servers() -> Result<Map<String, Value>, AppError> {
    let config = read_opencode_config()?;
    Ok(config
        .get("mcp")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default())
}

pub fn set_mcp_server(id: &str, config: Value) -> Result<(), AppError> {
    cst_set_object_property("mcp", id, &config)
}

pub fn remove_mcp_server(id: &str) -> Result<(), AppError> {
    cst_remove_object_property("mcp", id)
}

// ---------------------------------------------------------------------------
// Plugin operations (CST-based — preserves comments)
// ---------------------------------------------------------------------------

pub fn add_plugin(plugin_name: &str) -> Result<(), AppError> {
    let raw = read_config_raw()?;
    let root = parse_cst(&raw)?;
    let root_obj = root.object_value_or_set();

    let plugins = root_obj.array_value_or_set("plugin");

    // Mutual exclusion: standard OMO and OMO Slim cannot coexist as plugins
    if plugin_name.starts_with("oh-my-opencode") && !plugin_name.starts_with("oh-my-opencode-slim")
    {
        // Adding standard OMO -> remove all Slim variants
        let to_remove: Vec<_> = plugins
            .elements()
            .into_iter()
            .filter(|el| {
                el.as_string_lit()
                    .and_then(|s| s.decoded_value().ok())
                    .map(|s| s.starts_with("oh-my-opencode-slim"))
                    .unwrap_or(false)
            })
            .collect();
        for node in to_remove {
            node.remove();
        }
    } else if plugin_name.starts_with("oh-my-opencode-slim") {
        // Adding Slim -> remove all standard OMO variants (but keep slim)
        let to_remove: Vec<_> = plugins
            .elements()
            .into_iter()
            .filter(|el| {
                el.as_string_lit()
                    .and_then(|s| s.decoded_value().ok())
                    .map(|s| {
                        s.starts_with("oh-my-opencode") && !s.starts_with("oh-my-opencode-slim")
                    })
                    .unwrap_or(false)
            })
            .collect();
        for node in to_remove {
            node.remove();
        }
    }

    // Check for duplicates
    let already_exists = plugins.elements().iter().any(|el| {
        el.as_string_lit()
            .and_then(|s| s.decoded_value().ok())
            .map(|s| s == plugin_name)
            .unwrap_or(false)
    });

    if !already_exists {
        plugins.append(CstInputValue::String(plugin_name.to_string()));
    }

    write_config_raw(&root.to_string())
}

pub fn remove_plugin_by_prefix(prefix: &str) -> Result<(), AppError> {
    let raw = read_config_raw()?;
    let root = parse_cst(&raw)?;
    let root_obj = root.object_value_or_set();

    if let Some(plugins) = root_obj.array_value("plugin") {
        let to_remove: Vec<_> = plugins
            .elements()
            .into_iter()
            .filter(|el| {
                el.as_string_lit()
                    .and_then(|s| s.decoded_value().ok())
                    .map(|s| {
                        if !s.starts_with(prefix) {
                            return false;
                        }
                        let rest = &s[prefix.len()..];
                        !rest.starts_with('-')
                    })
                    .unwrap_or(false)
            })
            .collect();
        for node in to_remove {
            node.remove();
        }

        // Remove empty plugin array
        if plugins.elements().is_empty() {
            if let Some(prop) = root_obj.get("plugin") {
                prop.remove();
            }
        }
    }

    write_config_raw(&root.to_string())
}
