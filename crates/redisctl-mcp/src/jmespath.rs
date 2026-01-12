//! JMESPath query support for MCP tools
//!
//! Provides query transformation for tool results and introspection
//! of available JMESPath functions.

use jmespath::Runtime;
use jmespath_extensions::registry::{Category, FunctionInfo, FunctionRegistry};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::OnceLock;

/// Global JMESPath runtime with all extensions registered
static RUNTIME: OnceLock<Runtime> = OnceLock::new();

/// Global function registry for introspection
static REGISTRY: OnceLock<FunctionRegistry> = OnceLock::new();

/// Get the global JMESPath runtime
fn runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| {
        let mut runtime = Runtime::new();
        runtime.register_builtin_functions();
        jmespath_extensions::register_all(&mut runtime);
        runtime
    })
}

/// Get the global function registry
fn registry() -> &'static FunctionRegistry {
    REGISTRY.get_or_init(|| {
        let mut reg = FunctionRegistry::new();
        reg.register_all();
        reg
    })
}

/// Apply a JMESPath query to a JSON value
pub fn apply_query(value: &Value, query: &str) -> Result<Value, String> {
    let expr = runtime()
        .compile(query)
        .map_err(|e| format!("Failed to compile query: {}", e))?;

    let result = expr
        .search(value)
        .map_err(|e| format!("Failed to execute query: {}", e))?;

    // Convert jmespath::Variable back to serde_json::Value
    let json_str =
        serde_json::to_string(&result).map_err(|e| format!("Failed to serialize result: {}", e))?;
    serde_json::from_str(&json_str).map_err(|e| format!("Failed to parse result: {}", e))
}

/// Serializable function info for MCP responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDetail {
    pub name: String,
    pub category: String,
    pub description: String,
    pub signature: String,
    pub example: String,
    pub is_standard: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jep: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
}

impl From<&FunctionInfo> for FunctionDetail {
    fn from(info: &FunctionInfo) -> Self {
        Self {
            name: info.name.to_string(),
            category: format!("{:?}", info.category),
            description: info.description.to_string(),
            signature: info.signature.to_string(),
            example: info.example.to_string(),
            is_standard: info.is_standard,
            jep: info.jep.map(|s| s.to_string()),
            aliases: info.aliases.iter().map(|s| s.to_string()).collect(),
        }
    }
}

/// List all available JMESPath function categories
pub fn list_categories() -> Vec<String> {
    Category::all().iter().map(|c| format!("{:?}", c)).collect()
}

/// List functions, optionally filtered by category
pub fn list_functions(category: Option<&str>) -> Vec<FunctionDetail> {
    let reg = registry();

    match category {
        Some(cat_name) => {
            // Try to match category name
            if let Some(cat) = parse_category(cat_name) {
                reg.functions_in_category(cat)
                    .map(FunctionDetail::from)
                    .collect()
            } else {
                vec![]
            }
        }
        None => reg.functions().map(FunctionDetail::from).collect(),
    }
}

/// Get detailed info about a specific function
pub fn get_function(name: &str) -> Option<FunctionDetail> {
    registry()
        .get_function_by_name_or_alias(name)
        .map(FunctionDetail::from)
}

/// Evaluate a JMESPath expression against provided input
pub fn evaluate(input: &str, expression: &str) -> Result<Value, String> {
    let value: Value =
        serde_json::from_str(input).map_err(|e| format!("Failed to parse input JSON: {}", e))?;

    apply_query(&value, expression)
}

/// Parse category string to Category enum
fn parse_category(name: &str) -> Option<Category> {
    match name.to_lowercase().as_str() {
        "standard" => Some(Category::Standard),
        "string" => Some(Category::String),
        "array" => Some(Category::Array),
        "object" => Some(Category::Object),
        "math" => Some(Category::Math),
        "type" => Some(Category::Type),
        "utility" => Some(Category::Utility),
        "validation" => Some(Category::Validation),
        "path" => Some(Category::Path),
        "expression" => Some(Category::Expression),
        "text" => Some(Category::Text),
        "hash" => Some(Category::Hash),
        "encoding" => Some(Category::Encoding),
        "regex" => Some(Category::Regex),
        "url" => Some(Category::Url),
        "uuid" => Some(Category::Uuid),
        "rand" => Some(Category::Rand),
        "datetime" => Some(Category::Datetime),
        "fuzzy" => Some(Category::Fuzzy),
        "phonetic" => Some(Category::Phonetic),
        "geo" => Some(Category::Geo),
        "semver" => Some(Category::Semver),
        "network" => Some(Category::Network),
        "ids" => Some(Category::Ids),
        "duration" => Some(Category::Duration),
        "color" => Some(Category::Color),
        "computing" => Some(Category::Computing),
        "multimatch" => Some(Category::MultiMatch),
        "jsonpatch" => Some(Category::Jsonpatch),
        "format" => Some(Category::Format),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_query() {
        let value = serde_json::json!({"name": "test", "count": 42});
        let result = apply_query(&value, "name").unwrap();
        assert_eq!(result, serde_json::json!("test"));
    }

    #[test]
    fn test_list_categories() {
        let cats = list_categories();
        assert!(cats.contains(&"String".to_string()));
        assert!(cats.contains(&"Math".to_string()));
    }

    #[test]
    fn test_list_functions() {
        let funcs = list_functions(None);
        assert!(!funcs.is_empty());
    }

    #[test]
    fn test_get_function() {
        let func = get_function("upper").unwrap();
        assert_eq!(func.name, "upper");
        assert!(!func.description.is_empty());
    }

    #[test]
    fn test_evaluate() {
        let result = evaluate(r#"[1, 2, 3]"#, "sum(@)").unwrap();
        // jmespath returns floats for numeric operations
        assert_eq!(result.as_f64().unwrap(), 6.0);
    }
}
