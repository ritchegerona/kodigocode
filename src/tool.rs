use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::{OwnedRwLockReadGuard, OwnedRwLockWriteGuard, RwLock};

tokio::task_local! {
    static TOOL_EXECUTION_LOCK_HELD: ();
    static TOOL_CALL_LOCK_HELD: ();
}

/// Metadata for a registered tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    pub source: String,
}

/// Capabilities that a tool may have or require.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolCapability {
    ReadOnly,
    WritesFiles,
    ExecutesCode,
    Network,
    Sandboxable,
    RequiresApproval,
}

/// Errors that can occur during tool execution.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ToolError {
    #[error("Failed to validate input: {message}")]
    InvalidInput { message: String },
    #[error("Failed to validate input: missing required field '{field}'")]
    MissingField { field: String },
    #[error("Failed to resolve path '{}': path escapes workspace", path.display())]
    PathEscape { path: PathBuf },
    #[error("Failed to execute tool: {message}")]
    ExecutionFailed { message: String },
    #[error("Failed to execute tool: operation timed out after {seconds}s")]
    Timeout { seconds: u64 },
    #[error("Tool execution cancelled: {message}")]
    Cancelled { message: String },
    #[error("Failed to locate tool: {message}")]
    NotAvailable { message: String },
    #[error("Failed to authorize tool execution: {message}")]
    PermissionDenied { message: String },
}

impl ToolError {
    pub fn invalid_input(msg: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: msg.into(),
        }
    }

    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField {
            field: field.into(),
        }
    }

    pub fn execution_failed(msg: impl Into<String>) -> Self {
        Self::ExecutionFailed {
            message: msg.into(),
        }
    }

    pub fn cancelled(msg: impl Into<String>) -> Self {
        Self::Cancelled {
            message: msg.into(),
        }
    }

    pub fn path_escape(path: impl Into<PathBuf>) -> Self {
        Self::PathEscape { path: path.into() }
    }

    pub fn not_available(msg: impl Into<String>) -> Self {
        Self::NotAvailable {
            message: msg.into(),
        }
    }

    pub fn permission_denied(msg: impl Into<String>) -> Self {
        Self::PermissionDenied {
            message: msg.into(),
        }
    }
}

/// Result of a tool execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub content: String,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl ToolResult {
    pub fn success(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            success: true,
            metadata: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            content: message.into(),
            success: false,
            metadata: None,
        }
    }

    pub fn json<T: Serialize>(value: &T) -> std::result::Result<Self, serde_json::Error> {
        Ok(Self {
            content: serde_json::to_string(value)?,
            success: true,
            metadata: None,
        })
    }

    pub fn with_metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Name the JSON type of a value the way a tool schema would spell it.
pub fn json_type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

pub fn value_preview(value: &Value) -> String {
    let preview = value.to_string();
    if preview.chars().count() > 120 {
        preview.chars().take(117).collect::<String>() + "..."
    } else {
        preview
    }
}

pub fn type_mismatch(field: &str, value: &Value, expected: &str) -> ToolError {
    ToolError::invalid_input(format!(
        "field '{field}' must be {expected}; got {}. Received: {}",
        json_type_name(value),
        value_preview(value)
    ))
}

fn is_absent(value: Option<&Value>) -> bool {
    matches!(value, None | Some(Value::Null))
}

/// Extract a required string field from JSON input.
pub fn required_str<'a>(input: &'a Value, field: &str) -> std::result::Result<&'a str, ToolError> {
    if let Some(value) = input.get(field) {
        if let Some(string_value) = value.as_str() {
            return Ok(string_value);
        }
        return Err(ToolError::invalid_input(format!(
            "field '{}' must be a string; got {}. Received: {}",
            field,
            json_type_name(value),
            value_preview(value)
        )));
    }

    let provided: Vec<&str> = input
        .as_object()
        .map(|obj| obj.keys().map(|k| k.as_str()).collect())
        .unwrap_or_default();
    if provided.is_empty() {
        Err(ToolError::missing_field(field))
    } else {
        let hint = format!(
            "missing required field '{}'. Input provided: {}",
            field,
            provided.join(", ")
        );
        Err(ToolError::invalid_input(hint))
    }
}

/// Extract an optional string field from JSON input.
pub fn optional_str<'a>(
    input: &'a Value,
    field: &str,
) -> std::result::Result<Option<&'a str>, ToolError> {
    let value = input.get(field);
    if is_absent(value) {
        return Ok(None);
    }
    let value = value.expect("is_absent covers the None case");
    value
        .as_str()
        .map(Some)
        .ok_or_else(|| ToolError::invalid_input(format!(
            "field '{}' must be a string; got {}.",
            field,
            json_type_name(value)
        )))
}

/// Extract a required u64 field from JSON input.
pub fn required_u64(input: &Value, field: &str) -> std::result::Result<u64, ToolError> {
    let value = input.get(field);
    if is_absent(value) {
        return Err(ToolError::missing_field(field));
    }
    let value = value.expect("is_absent covers the None case");
    value.as_u64().ok_or_else(|| ToolError::invalid_input(format!(
        "field '{}' must be a non-negative integer; got {}.",
        field,
        json_type_name(value)
    )))
}

/// Extract an optional u64 field with default.
pub fn optional_u64(
    input: &Value,
    field: &str,
    default: u64,
) -> std::result::Result<u64, ToolError> {
    let value = input.get(field);
    if is_absent(value) {
        return Ok(default);
    }
    let value = value.expect("is_absent covers the None case");
    value.as_u64().ok_or_else(|| ToolError::invalid_input(format!(
        "field '{}' must be a non-negative integer; got {}.",
        field,
        json_type_name(value)
    )))
}

/// Extract an optional bool field with default.
pub fn optional_bool(
    input: &Value,
    field: &str,
    default: bool,
) -> std::result::Result<bool, ToolError> {
    Ok(optional_bool_opt(input, field)?.unwrap_or(default))
}

/// Extract an optional bool that has no default.
pub fn optional_bool_opt(
    input: &Value,
    field: &str,
) -> std::result::Result<Option<bool>, ToolError> {
    let value = input.get(field);
    if is_absent(value) {
        return Ok(None);
    }
    let value = value.expect("is_absent covers the None case");
    value.as_bool().map(Some).ok_or_else(|| {
        ToolError::invalid_input(format!(
            "field '{}' must be a boolean; got {}.",
            field,
            json_type_name(value)
        ))
    })
}

/// Descriptor that describes a tool available in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDescriptor {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    pub output_schema: Value,
    pub supports_parallel_tool_calls: bool,
    pub timeout_ms: Option<u64>,
    pub capabilities: Vec<ToolCapability>,
    pub is_mutating: bool,
}

impl ToolDescriptor {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            input_schema: serde_json::json!({}),
            output_schema: serde_json::json!({}),
            supports_parallel_tool_calls: false,
            timeout_ms: None,
            capabilities: vec![],
            is_mutating: false,
        }
    }

    pub fn with_input_schema(mut self, schema: Value) -> Self {
        self.input_schema = schema;
        self
    }

    pub fn with_output_schema(mut self, schema: Value) -> Self {
        self.output_schema = schema;
        self
    }

    pub fn with_parallel(mut self) -> Self {
        self.supports_parallel_tool_calls = true;
        self
    }

    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }

    pub fn with_capability(mut self, cap: ToolCapability) -> Self {
        self.capabilities.push(cap);
        self
    }

    pub fn mutating(mut self) -> Self {
        self.is_mutating = true;
        self
    }
}

/// A wrapped descriptor together with its runtime configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfiguredToolDescriptor {
    pub spec: ToolDescriptor,
    pub supports_parallel_tool_calls: bool,
}

/// Abstraction for a pluggable tool.
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str {
        ""
    }
    fn source(&self) -> &'static str {
        "built-in"
    }
    fn descriptor(&self) -> ToolDescriptor {
        ToolDescriptor::new(self.name(), self.description())
    }
    fn is_mutating(&self) -> bool {
        false
    }
    async fn run_typed(&self, input: &Value) -> std::result::Result<ToolResult, ToolError>;
    async fn run(&self, args: &[String]) -> Result<String>;
}

/// Tool execution guard controls concurrent vs. serial execution.
#[derive(Debug)]
enum ToolExecutionGuard {
    Parallel(#[allow(dead_code)] OwnedRwLockReadGuard<()>),
    Serial(#[allow(dead_code)] OwnedRwLockWriteGuard<()>),
    Reentrant,
}

/// Manages concurrent tool execution.
#[derive(Debug, Default)]
pub struct ToolCallRuntime {
    execution_lock: Arc<RwLock<()>>,
}

impl ToolCallRuntime {
    async fn acquire(&self, supports_parallel: bool) -> ToolExecutionGuard {
        if TOOL_EXECUTION_LOCK_HELD.try_with(|_| ()).is_ok() {
            return ToolExecutionGuard::Reentrant;
        }
        if supports_parallel {
            ToolExecutionGuard::Parallel(self.execution_lock.clone().read_owned().await)
        } else {
            ToolExecutionGuard::Serial(self.execution_lock.clone().write_owned().await)
        }
    }
}

/// A tool invocation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: Value,
    pub raw_tool_call_id: Option<String>,
}

/// Simple registry that holds boxed tools indexed by name.
pub struct ToolRegistry {
    tools: std::collections::HashMap<String, Box<dyn Tool>>,
    specs: HashMap<String, ConfiguredToolDescriptor>,
    runtime: ToolCallRuntime,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: std::collections::HashMap::new(),
            specs: HashMap::new(),
            runtime: ToolCallRuntime::default(),
        }
    }

    pub fn register<T: Tool + 'static>(&mut self, tool: T) {
        let desc = tool.descriptor();
        let name = tool.name().to_string();
        self.specs.insert(
            name.clone(),
            ConfiguredToolDescriptor {
                supports_parallel_tool_calls: desc.supports_parallel_tool_calls,
                spec: desc,
            },
        );
        self.tools.insert(name, Box::new(tool));
    }

    pub fn register_boxed(&mut self, tool: Box<dyn Tool>) {
        let desc = tool.descriptor();
        let name = tool.name().to_string();
        self.specs.insert(
            name.clone(),
            ConfiguredToolDescriptor {
                supports_parallel_tool_calls: desc.supports_parallel_tool_calls,
                spec: desc,
            },
        );
        self.tools.insert(name, tool);
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn Tool>> {
        self.tools.get(name)
    }

    pub fn list(&self) -> Vec<ToolInfo> {
        let mut infos: Vec<ToolInfo> = self
            .tools
            .iter()
            .map(|(name, tool)| ToolInfo {
                name: name.clone(),
                description: tool.description().to_string(),
                source: tool.source().to_string(),
            })
            .collect();
        infos.sort_by(|a, b| a.name.cmp(&b.name));
        infos
    }

    pub fn list_specs(&self) -> Vec<ConfiguredToolDescriptor> {
        self.specs.values().cloned().collect()
    }

    /// Dispatch a tool call with approval and timeout.
    pub async fn dispatch(
        &self,
        call: &ToolCall,
        allow_mutating: bool,
    ) -> std::result::Result<ToolResult, ToolError> {
        let configured = self.specs.get(&call.name).cloned().ok_or_else(|| {
            ToolError::not_available(format!("Tool spec '{}' not found", call.name))
        })?;

        let is_mutating = configured.spec.is_mutating;

        if is_mutating && !allow_mutating {
            return Err(ToolError::permission_denied(format!(
                "Tool '{}' requires mutating permission",
                call.name
            )));
        }

        let _guard = self
            .runtime
            .acquire(configured.supports_parallel_tool_calls)
            .await;

        let result = if let Some(timeout_ms) = configured.spec.timeout_ms {
            match tokio::time::timeout(
                Duration::from_millis(timeout_ms),
                self.execute_inner(&call.name, &call.arguments),
            )
            .await
            {
                Ok(r) => r,
                Err(_) => Err(ToolError::Timeout {
                    seconds: timeout_ms / 1000,
                }),
            }
        } else {
            self.execute_inner(&call.name, &call.arguments).await
        };

        result
    }

    async fn execute_inner(
        &self,
        name: &str,
        arguments: &Value,
    ) -> std::result::Result<ToolResult, ToolError> {
        let tool = self.tools.get(name).ok_or_else(|| {
            ToolError::not_available(format!("Tool '{}' not found", name))
        })?;
        TOOL_CALL_LOCK_HELD
            .scope((), tool.run_typed(arguments))
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn tool_result_success_sets_plain_content() {
        let content = "op completed";
        let result = ToolResult::success(content);
        assert!(result.success);
        assert_eq!(result.content, content);
        assert!(result.metadata.is_none());
    }

    #[test]
    fn tool_result_json_round_trips() {
        let result = ToolResult::json(&json!({"ok": true})).expect("json");
        assert!(result.success);
        let content: serde_json::Value =
            serde_json::from_str(&result.content).expect("json parse");
        assert_eq!(content, json!({"ok": true}));
    }

    #[test]
    fn extractors_validate_shape() {
        let input = json!({"name": "demo", "count": 7, "enabled": true});
        assert_eq!(required_str(&input, "name").expect("name"), "demo");
        assert_eq!(optional_str(&input, "name").unwrap(), Some("demo"));
        assert_eq!(optional_str(&input, "missing").unwrap(), None);
        assert_eq!(optional_str(&json!({"name": null}), "name").unwrap(), None);
        assert_eq!(optional_u64(&input, "count", 0).unwrap(), 7);
        assert!(optional_bool(&input, "enabled", false).unwrap());
        let err = required_u64(&input, "name")
            .expect_err("string is not u64")
            .to_string();
        assert!(
            err.contains("field 'name' must be a non-negative integer"),
            "{err}"
        );
    }

    #[test]
    fn optional_extractors_refuse_type_mismatches() {
        let err = optional_bool(&json!({"dry_run": "true"}), "dry_run", false)
            .expect_err("stringy bool must not become default")
            .to_string();
        assert!(err.contains("dry_run"), "{err}");
        assert!(err.contains("must be a boolean"), "{err}");
        assert!(err.contains("got string"), "{err}");

        assert!(
            optional_bool(
                &json!({"flag": serde_json::Value::Bool(true)}),
                "flag",
                false
            )
            .unwrap()
        );
        assert!(optional_bool(&json!({"flag": null}), "flag", true).unwrap());
        assert_eq!(optional_u64(&json!({"n": null}), "n", 42).unwrap(), 42);
        assert_eq!(optional_str(&json!({"s": null}), "s").unwrap(), None);
    }

    #[test]
    fn required_str_reports_provided_fields_on_missing() {
        let input = json!({"path": "src/lib.rs", "content": "new body"});
        let err = required_str(&input, "replace").expect_err("replace is missing");
        let message = err.to_string();
        assert!(message.contains("missing required field 'replace'"));
        assert!(message.contains("Input provided:"));
        assert!(message.contains("path"));
        assert!(message.contains("content"));
    }

    #[test]
    fn tool_error_display() {
        let err = ToolError::missing_field("path");
        assert_eq!(
            err.to_string(),
            "Failed to validate input: missing required field 'path'"
        );
    }
}