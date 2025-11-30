use crate::AFastData;
use crate::AFastKind;
use crate::HandlerGeneric;

use super::Field;
use super::Kind;
use super::Tag;

/// Generate OpenAPI 3.0 specification for a single service
/// Takes the service documentation JSON and converts it to OpenAPI format
pub fn gen_swagger_service(
    service_name: &str,
    service_desc: &str,
    base_url: &str,
    doc_json: &str,
) -> String {
    // Parse the existing documentation JSON
    let handlers_doc: Vec<serde_json::Value> = match serde_json::from_str(doc_json) {
        Ok(d) => d,
        Err(_) => return gen_empty_openapi(service_name, service_desc, base_url),
    };

    let mut paths_parts = Vec::new();
    let mut schema_parts = Vec::new();
    let mut schema_counter = 0;

    for handler_doc in handlers_doc {
        let handler_name = handler_doc["name"].as_str().unwrap_or("");
        let description = handler_doc["desc"].as_str().unwrap_or("");
        
        if handler_name.is_empty() {
            continue;
        }

        // Generate schema names
        let request_schema_name = format!("{}Request_{}", service_name, handler_name);
        let response_schema_name = format!("{}Response_{}", service_name, handler_name);
        schema_counter += 1;

        // Generate request schema
        let request_schema = gen_openapi_schema_from_doc(&handler_doc["request"]);
        schema_parts.push(format!(
            r#""{}": {}"#,
            request_schema_name,
            request_schema
        ));

        // Generate response schema
        let response_schema = gen_openapi_schema_from_doc(&handler_doc["response"]);
        schema_parts.push(format!(
            r#""{}": {}"#,
            response_schema_name,
            response_schema
        ));

        // Build path
        let path = format!("/api/{}", handler_name);
        let operation_id = format!("{}_{}", service_name, handler_name);

        // Build operation
        let operation = format!(
            concat!(
                "{{\n",
                "  \"operationId\": \"{}\",\n",
                "  \"summary\": {},\n",
                "  \"description\": {},\n",
                "  \"tags\": [\"{}\"],\n",
                "  \"requestBody\": {{\n",
                "    \"required\": true,\n",
                "    \"content\": {{\n",
                "      \"application/json\": {{\n",
                "        \"schema\": {{\n",
                "          \"$ref\": \"#/components/schemas/{}\"\n",
                "        }}\n",
                "      }}\n",
                "    }}\n",
                "  }},\n",
                "  \"responses\": {{\n",
                "    \"200\": {{\n",
                "      \"description\": \"Successful response\",\n",
                "      \"content\": {{\n",
                "        \"application/json\": {{\n",
                "          \"schema\": {{\n",
                "            \"$ref\": \"#/components/schemas/{}\"\n",
                "          }}\n",
                "        }}\n",
                "      }}\n",
                "    }},\n",
                "    \"400\": {{\n",
                "      \"description\": \"Bad request - Invalid input or validation error\"\n",
                "    }},\n",
                "    \"500\": {{\n",
                "      \"description\": \"Internal server error\"\n",
                "    }}\n",
                "  }}\n",
                "}}"
            ),
            operation_id,
            escape_json_string(description),
            escape_json_string(description),
            service_name,
            request_schema_name,
            response_schema_name
        );

        paths_parts.push(format!(
            concat!(
                "\"{}\": {{\n",
                "  \"post\": {}\n",
                "}}"
            ),
            path, operation
        ));
    }

    // Build OpenAPI spec
    format!(
        concat!(
            "{{\n",
            "  \"openapi\": \"3.0.0\",\n",
            "  \"info\": {{\n",
            "    \"title\": \"{} API\",\n",
            "    \"version\": \"1.0.0\",\n",
            "    \"description\": {}\n",
            "  }},\n",
            "  \"servers\": [\n",
            "    {{\n",
            "      \"url\": \"{}\",\n",
            "      \"description\": \"AFast API Server\"\n",
            "    }}\n",
            "  ],\n",
            "  \"tags\": [\n",
            "    {{\n",
            "      \"name\": \"{}\",\n",
            "      \"description\": {}\n",
            "    }}\n",
            "  ],\n",
            "  \"paths\": {{\n",
            "{}\n",
            "  }},\n",
            "  \"components\": {{\n",
            "    \"schemas\": {{\n",
            "{}\n",
            "    }}\n",
            "  }}\n",
            "}}"
        ),
        service_name,
        escape_json_string(service_desc),
        base_url,
        service_name,
        escape_json_string(service_desc),
        paths_parts.join(",\n"),
        schema_parts.join(",\n")
    )
}

/// Generate OpenAPI schema from AFast doc field
fn gen_openapi_schema_from_doc(field: &serde_json::Value) -> String {
    if let Some(kind_obj) = field.get("kind") {
        let kind_str = kind_obj.as_str().unwrap_or("object");
        
        match kind_str {
            "object" => {
                if let Some(fields) = field.get("fields") {
                    if let Some(fields_array) = fields.as_array() {
                        let mut properties_parts = Vec::new();
                        let mut required_parts = Vec::new();

                        for f in fields_array {
                            let field_name = f["name"].as_str().unwrap_or("");
                            if field_name.is_empty() {
                                continue;
                            }

                            let mut field_schema = gen_openapi_type_from_kind(f.get("kind"));
                            
                            // Add description and validation from tag
                            if let Some(tag) = f.get("tag") {
                                if let Some(desc) = tag.get("description") {
                                    if let Some(desc_str) = desc.as_str() {
                                        field_schema = add_description(&field_schema, desc_str);
                                    }
                                }
                                if tag.get("required").is_some() {
                                    required_parts.push(escape_json_string(field_name));
                                }
                                if let Some(min) = tag.get("min") {
                                    if let Some(min_obj) = min.as_object() {
                                        if let Some(val) = min_obj.get("value") {
                                            if let Some(num) = val.as_i64() {
                                                field_schema = add_minimum(&field_schema, num);
                                            }
                                        }
                                    }
                                }
                                if let Some(max) = tag.get("max") {
                                    if let Some(max_obj) = max.as_object() {
                                        if let Some(val) = max_obj.get("value") {
                                            if let Some(num) = val.as_i64() {
                                                field_schema = add_maximum(&field_schema, num);
                                            }
                                        }
                                    }
                                }
                            }

                            properties_parts.push(format!(
                                r#"    "{}": {}"#,
                                field_name,
                                field_schema
                            ));
                        }

                        let mut schema = format!(
                            concat!(
                                "{{\n",
                                "  \"type\": \"object\",\n",
                                "  \"properties\": {{\n",
                                "{}\n",
                                "  }}"
                            ),
                            properties_parts.join(",\n")
                        );

                        if !required_parts.is_empty() {
                            schema.push_str(&format!(
                                ",\n  \"required\": [{}]",
                                required_parts.join(", ")
                            ));
                        }

                        schema.push_str("\n}");
                        schema
                    } else {
                        r#"{"type": "object"}"#.to_string()
                    }
                } else {
                    r#"{"type": "object"}"#.to_string()
                }
            }
            _ => gen_openapi_type_from_kind(Some(kind_obj)),
        }
    } else {
        r#"{"type": "object"}"#.to_string()
    }
}

/// Generate OpenAPI type from kind string
fn gen_openapi_type_from_kind(kind: Option<&serde_json::Value>) -> String {
    let kind_str = kind.and_then(|k| k.as_str()).unwrap_or("string");
    
    match kind_str {
        "string" => r#"{"type": "string"}"#.to_string(),
        "i8" | "i16" | "i32" => r#"{"type": "integer", "format": "int32"}"#.to_string(),
        "i64" => r#"{"type": "integer", "format": "int64"}"#.to_string(),
        "u8" | "u16" | "u32" => r#"{"type": "integer", "format": "int32"}"#.to_string(),
        "u64" => r#"{"type": "integer", "format": "int64"}"#.to_string(),
        "f32" => r#"{"type": "number", "format": "float"}"#.to_string(),
        "f64" => r#"{"type": "number", "format": "double"}"#.to_string(),
        "bool" => r#"{"type": "boolean"}"#.to_string(),
        "array" => {
            // Simplified - would need to handle items recursively
            r#"{"type": "array", "items": {"type": "string"}}"#.to_string()
        }
        _ => r#"{"type": "object"}"#.to_string(),
    }
}

/// Add description to schema JSON
fn add_description(schema: &str, desc: &str) -> String {
    if schema.contains("\"description\"") {
        return schema.to_string();
    }
    
    // Simple insertion before closing brace
    let desc_escaped = escape_json_string(desc);
    schema.replace("}", &format!(r#",
  "description": {}
}}"#, desc_escaped))
}

/// Add minimum to schema JSON
fn add_minimum(schema: &str, min: i64) -> String {
    if schema.contains("\"minimum\"") {
        return schema.to_string();
    }
    
    schema.replace("}", &format!(r#",
  "minimum": {}
}}"#, min))
}

/// Add maximum to schema JSON
fn add_maximum(schema: &str, max: i64) -> String {
    if schema.contains("\"maximum\"") {
        return schema.to_string();
    }
    
    schema.replace("}", &format!(r#",
  "maximum": {}
}}"#, max))
}

/// Escape string for JSON
fn escape_json_string(s: &str) -> String {
    format!("\"{}\"", s.replace("\\", "\\\\").replace("\"", "\\\"").replace("\n", "\\n"))
}

/// Generate empty OpenAPI spec
fn gen_empty_openapi(service_name: &str, service_desc: &str, base_url: &str) -> String {
    format!(
        concat!(
            "{{\n",
            "  \"openapi\": \"3.0.0\",\n",
            "  \"info\": {{\n",
            "    \"title\": \"{} API\",\n",
            "    \"version\": \"1.0.0\",\n",
            "    \"description\": {}\n",
            "  }},\n",
            "  \"servers\": [\n",
            "    {{\n",
            "      \"url\": \"{}\",\n",
            "      \"description\": \"AFast API Server\"\n",
            "    }}\n",
            "  ],\n",
            "  \"paths\": {{}},\n",
            "  \"components\": {{\n",
            "    \"schemas\": {{}}\n",
            "  }}\n",
            "}}"
        ),
        service_name,
        escape_json_string(service_desc),
        base_url
    )
}
