# Proxy (HTTP Proxy Server)

## Overview

Local HTTP proxy server that intercepts API requests, provides failover between providers, circuit breaker protection, and request/response processing. Built with `hyper` for async HTTP handling.

## Module Index

### Core Infrastructure
| Module | Purpose |
|--------|---------|
| `server.rs` | HTTP server setup and lifecycle |
| `handlers.rs` | Request routing and handling |
| `handler_config.rs` | Handler configuration |
| `handler_context.rs` | Request context management |
| `forwarder.rs` | Request forwarding to upstream |
| `http_client.rs` | HTTP client for upstream requests |

### Routing & Failover
| Module | Purpose |
|--------|---------|
| `provider_router.rs` | Provider selection and routing |
| `providers.rs` | Provider definitions and configs |
| `failover_switch.rs` | Failover logic between providers |
| `circuit_breaker.rs` | Circuit breaker pattern implementation |
| `health.rs` | Provider health checking |

### Request/Response Processing
| Module | Purpose |
|--------|---------|
| `body_filter.rs` | Request/response body filtering |
| `model_mapper.rs` | Model name mapping between providers |
| `response_handler.rs` | Response processing (stream/non-stream) |
| `response_processor.rs` | Response transformation |
| `thinking_rectifier.rs` | Thinking block processing |

### Error Handling & Logging
| Module | Purpose |
|--------|---------|
| `error.rs` | `ProxyError` type definition |
| `error_mapper.rs` | Error code mapping |
| `log_codes.rs` | Structured log codes |

### Session & Usage
| Module | Purpose |
|--------|---------|
| `session.rs` | Session ID extraction and management |
| `usage.rs` | Token usage tracking |

### Types
| Module | Purpose |
|--------|---------|
| `types.rs` | `ProxyConfig`, `ProxyStatus`, `ProxyServerInfo` |

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Incoming Request                        │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    server.rs (HTTP Server)                   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   handlers.rs (Request Router)               │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│               provider_router.rs (Provider Selection)        │
│  ┌─────────────────┐    ┌─────────────────┐                 │
│  │ circuit_breaker │◄──►│ failover_switch │                 │
│  └─────────────────┘    └─────────────────┘                 │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                  forwarder.rs (Upstream Request)             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │ body_filter  │  │ model_mapper │  │ http_client  │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│               response_handler.rs (Response Processing)      │
│  ┌──────────────────┐    ┌────────────────────────┐         │
│  │  StreamHandler   │    │   NonStreamHandler     │         │
│  └──────────────────┘    └────────────────────────┘         │
└─────────────────────────────────────────────────────────────┘
```

## Key Components

### Circuit Breaker
```rust
pub struct CircuitBreaker {
    state: CircuitState,           // Closed, Open, HalfOpen
    failure_count: u32,
    success_count: u32,
    config: CircuitBreakerConfig,
}

pub struct CircuitBreakerConfig {
    failure_threshold: u32,        // Failures before opening
    success_threshold: u32,        // Successes to close
    open_duration: Duration,       // Time before half-open
}
```

### Provider Router
```rust
pub struct ProviderRouter {
    providers: Vec<ProviderConfig>,
    circuit_breakers: HashMap<String, CircuitBreaker>,
    current_index: AtomicUsize,
}

impl ProviderRouter {
    pub async fn route(&self, request: &Request) -> Result<&ProviderConfig, ProxyError>;
    pub async fn mark_success(&self, provider_id: &str);
    pub async fn mark_failure(&self, provider_id: &str);
}
```

### Response Handlers
```rust
// For streaming responses (SSE)
pub struct StreamHandler;
impl StreamHandler {
    pub async fn handle(response: Response) -> Result<Response, ProxyError>;
}

// For non-streaming responses
pub struct NonStreamHandler;
impl NonStreamHandler {
    pub async fn handle(response: Response) -> Result<Response, ProxyError>;
}
```

## Session Management

Session IDs are extracted from various sources:
```rust
pub enum SessionIdSource {
    Header,           // X-Session-ID header
    QueryParam,       // ?session_id=xxx
    Cookie,           // session_id cookie
    Generated,        // Auto-generated UUID
}

pub fn extract_session_id(request: &Request) -> SessionIdResult;
```

## When Making Changes

1. **New provider support**: Add to `providers.rs`, update `model_mapper.rs`
2. **Request modification**: Update `body_filter.rs` or `forwarder.rs`
3. **Response modification**: Update `response_processor.rs`
4. **Failover logic**: Modify `failover_switch.rs` and `circuit_breaker.rs`
5. **New error type**: Add to `error.rs`, map in `error_mapper.rs`
6. **Usage tracking**: Update `usage.rs`
