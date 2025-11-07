use std::sync::Arc;

use crate::{AFastData, HandlerGeneric};

/// Builds JavaScript object structure from handlers grouped by namespace
///
/// # Arguments
/// * `handlers` - Reference to Arc wrapped vector of HandlerGeneric instances
///
/// # Returns
/// * JavaScript code string with nested namespace structure
pub fn simple_js_builder<T, H>(handlers: &Arc<Vec<HandlerGeneric<T, H>>>, js: bool) -> String
where
    T: Send + Sync + 'static,
    H: AFastData,
{
    // Group handlers by namespace
    let mut groups: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for handler in handlers.iter() {
        let namespace_key = handler.namespace.join(".");
        groups
            .entry(namespace_key)
            .or_insert_with(Vec::new)
            .push(if js {
                handler.js.clone()
            } else {
                handler.ts.clone()
            });
    }

    build_js_object(&groups, "", true)
}

/// Recursively builds JavaScript object structure from grouped handlers
///
/// # Arguments
/// * `groups` - Handlers grouped by namespace
/// * `current_ns` - Current namespace being processed
/// * `is_outermost` - Whether this is the outermost level (uses '=' instead of ':')
///
/// # Returns
/// * Formatted JavaScript code string
fn build_js_object(
    groups: &std::collections::HashMap<String, Vec<String>>,
    current_ns: &str,
    is_outermost: bool,
) -> String {
    let mut result = String::new();
    let mut first = true;

    // Add functions for current namespace
    if let Some(js_codes) = groups.get(current_ns) {
        for (i, code) in js_codes.iter().enumerate() {
            if i > 0 {
                result.push_str(",\n");
            }
            result.push_str(code);
            first = false;
        }
    }

    // Process child namespaces
    let prefix = if current_ns.is_empty() {
        ""
    } else {
        &format!("{}.", current_ns)
    };

    let mut child_namespaces: Vec<_> = groups
        .keys()
        .filter(|ns| ns.starts_with(prefix) && ns != &current_ns)
        .collect();

    // Only process direct children (no deeper nesting in the namespace part)
    child_namespaces.retain(|ns| {
        let relative_ns = &ns[prefix.len()..];
        !relative_ns.contains('.')
    });

    // Sort namespaces alphabetically for consistent output
    child_namespaces.sort();

    for ns in child_namespaces {
        if !first {
            result.push_str(",\n");
        }

        let child_name = &ns[prefix.len()..];

        // Use '=' for outermost level, ':' for inner levels
        let separator = if is_outermost && current_ns.is_empty() {
            " = "
        } else {
            ": "
        };

        result.push_str(child_name);
        result.push_str(separator);
        result.push_str("{\n");

        let child_js = build_js_object(groups, ns, false);
        result.push_str(&child_js);

        result.push_str("\n}");

        first = false;
    }

    // Return empty string if no content was added
    if first {
        return String::new();
    }

    result
}
