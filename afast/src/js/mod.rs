pub const JS: &str = r#"export class AFastByteBuffer{constructor(size=64){this.encoder=new TextEncoder();this.buf=new ArrayBuffer(size);this.view=new DataView(this.buf);this.length=0}ensure(size){if(this.length+size>this.buf.byteLength){let newSize=this.buf.byteLength*2;while(newSize<this.length+size)newSize*=2;const newBuf=new ArrayBuffer(newSize);new Uint8Array(newBuf).set(new Uint8Array(this.buf,0,this.length));this.buf=newBuf;this.view=new DataView(this.buf)}}pBy(b){this.ensure(1);this.view.setUint8(this.length,b);this.length+=1}pBys(arr){this.ensure(arr.length);new Uint8Array(this.buf,this.length,arr.length).set(arr);this.length+=arr.length}pB(b){this.pBy(b?1:0)}pI8(n){this.ensure(1);this.view.setInt8(this.length,n);this.length+=1}pU8(n){this.ensure(1);this.view.setUint8(this.length,n);this.length+=1}pI16(n){this.ensure(2);this.view.setInt16(this.length,n,false);this.length+=2}pU16(n){this.ensure(2);this.view.setUint16(this.length,n,false);this.length+=2}pI32(n){this.ensure(4);this.view.setInt32(this.length,n,false);this.length+=4}pU32(n){this.ensure(4);this.view.setUint32(this.length,n,false);this.length+=4}pI64(n){this.ensure(8);const high=Math.floor(n/2**32);const low=n>>>0;this.view.setInt32(this.length,high,false);this.view.setInt32(this.length+4,low,false);this.length+=8}pU64(n){this.ensure(8);const high=Math.floor(n/2**32);const low=n>>>0;this.view.setUint32(this.length,high,false);this.view.setUint32(this.length+4,low,false);this.length+=8}pI128(n){this.ensure(16);const highHigh=0;const highLow=Math.floor(n/2**32);const lowHigh=n>>>0;const lowLow=0;this.view.setUint32(this.length,highHigh,false);this.view.setUint32(this.length+4,highLow,false);this.view.setUint32(this.length+8,lowHigh,false);this.view.setUint32(this.length+12,lowLow,false);this.length+=16}pU128(n){this.ensure(16);const highHigh=0;const highLow=Math.floor(n/2**32);const lowHigh=n>>>0;const lowLow=0;this.view.setUint32(this.length,highHigh,false);this.view.setUint32(this.length+4,highLow,false);this.view.setUint32(this.length+8,lowHigh,false);this.view.setUint32(this.length+12,lowLow,false);this.length+=16}pF32(n){this.ensure(4);this.view.setFloat32(this.length,n,false);this.length+=4}pF64(n){this.ensure(8);this.view.setFloat64(this.length,n,false);this.length+=8}pS(str){const bytes=this.encoder.encode(str);this.pU32(bytes.length);this.pBys(bytes)}tU8A(){return new Uint8Array(this.buf,0,this.length)}}
export class AFastByteReader{constructor(uint8Array){this.buf=uint8Array.buffer;this.view=new DataView(this.buf,uint8Array.byteOffset,uint8Array.byteLength);this.offset=0}rBy(){if(this.offset>=this.view.byteLength){throw new Error('Out of range: trying to read byte, but no more bytes left');}const v=this.view.getUint8(this.offset);this.offset+=1;return v}rBys(len){if(this.offset+len>this.view.byteLength){throw new Error(`Out of range:trying to read ${len}bytes,but only ${this.view.byteLength-this.offset}left`)}const bytes=new Uint8Array(this.view.buffer,this.view.byteOffset+this.offset,len);this.offset+=len;return bytes}rB(){const v=this.view.getUint8(this.offset);this.offset+=1;return v!==0}rI8(){const v=this.view.getInt8(this.offset);this.offset+=1;return v}rU8(){const v=this.view.getUint8(this.offset);this.offset+=1;return v}rI16(){const v=this.view.getInt16(this.offset,false);this.offset+=2;return v}rU16(){const v=this.view.getUint16(this.offset,false);this.offset+=2;return v}rI32(){const v=this.view.getInt32(this.offset,false);this.offset+=4;return v}rU32(){const v=this.view.getUint32(this.offset,false);this.offset+=4;return v}rI64(){const v=Number(this.view.getBigInt64(this.offset,false));this.offset+=8;return v}rU64(){const v=Number(this.view.getBigUint64(this.offset,false));this.offset+=8;return v}rI128(){const high=Number(this.view.getBigInt64(this.offset,false));const low=Number(this.view.getBigUint64(this.offset+8,false));this.offset+=16;return high * Math.pow(2, 64) + low}rU128(){const high=Number(this.view.getBigUint64(this.offset,false));const low=Number(this.view.getBigUint64(this.offset+8,false));this.offset+=16;return high * Math.pow(2, 64) + low}rF32(){const v=this.view.getFloat32(this.offset,false);this.offset+=4;return v}rF64(){const v=this.view.getFloat64(this.offset,false);this.offset+=8;return v}rS(){const len=this.rU32();const bytes=this.rBys(len);const decoder=new TextDecoder();return decoder.decode(bytes)}eof(){return this.offset>=this.view.byteLength}}
export class AFastValidateError extends Error{constructor(message){super(message);this.name=this.constructor.name;if(Error.captureStackTrace){Error.captureStackTrace(this,this.constructor)}}}
"#;

use crate::AFastData;
use crate::AFastKind;
use crate::HandlerGeneric;

use super::Field;
use super::Kind;
use super::Tag;

impl Field {
    pub fn gen_js_to_bytes(&self, depth: usize) -> String {
        let kind_code = self.kind.gen_js_to_bytes(&self.name, depth);
        kind_code
    }

    pub fn gen_bytes_to_js(&self, depth: usize) -> String {
        let kind_code = format!("const {}={};", self.name, self.kind.gen_bytes_to_js(depth));
        kind_code
    }

    pub fn gen_js_type(&self) -> String {
        let kind_type = self.kind.gen_js_type();
        kind_type
    }

    pub fn gen_js_validate(&self, depth: usize) -> String {
        self.kind
            .gen_js_validate(&self.name, self.tag.as_ref(), depth)
    }
}

impl Kind {
    pub fn gen_js_to_bytes(&self, name: &str, depth: usize) -> String {
        match self {
            Kind::Unit => "".to_string(),
            Kind::I8 => format!("_b1.pI8({});", name),
            Kind::I16 => format!("_b1.pI16({});", name),
            Kind::I32 => format!("_b1.pI32({});", name),
            Kind::I64 => format!("_b1.pI64({});", name),
            Kind::I128 => format!("_b1.pI128({});", name),
            Kind::U8 => format!("_b1.pU8({});", name),
            Kind::U16 => format!("_b1.pU16({});", name),
            Kind::U32 => format!("_b1.pU32({});", name),
            Kind::U64 => format!("_b1.pU64({});", name),
            Kind::U128 => format!("_b1.pU128({});", name),
            Kind::F32 => format!("_b1.pF32({});", name),
            Kind::F64 => format!("_b1.pF64({});", name),
            Kind::Bool => format!("_b1.pB({});", name),
            Kind::String => format!("_b1.pS({});", name),
            Kind::Vec(kind) => {
                let mut code = String::new();
                code.push_str(&format!("_b1.pU32({}.length);", name));
                code.push_str(&format!(
                    "for(let _i{} of {}){{{}}}",
                    depth,
                    name,
                    kind.gen_js_to_bytes(&format!("_i{}", depth), depth + 1)
                ));
                code
            }
            Kind::Enum { variants } => {
                let mut code = String::new();
                code.push_str(&format!("_b1.pU32({}._type);", name));
                code.push_str(&format!("switch({}._type){{", name));
                for (i, variant) in variants.iter().enumerate() {
                    code.push_str(&format!(
                        "case {}:{}break;",
                        i,
                        variant.gen_js_to_bytes(name, depth)
                    ));
                }
                code.push_str("}");
                code
            }
            Kind::Struct { fields } => {
                let mut code = String::new();
                for field in fields {
                    code.push_str(
                        &field
                            .kind
                            .gen_js_to_bytes(&format!("{}.{}", name, field.name), depth + 1),
                    );
                }
                code
            }
            Kind::Tuple(elements) => {
                let mut code = String::new();
                for (i, elem) in elements.iter().enumerate() {
                    code.push_str(&elem.gen_js_to_bytes(&format!("{}[{}]", name, i), depth + 1));
                }
                code
            }
            Kind::Nullable(kind) => {
                let mut code = String::new();
                code.push_str(&format!(
                    "if({0}===null){{_b1.pU8(0);}}else{{_b1.pU8(1);{1}}}",
                    name,
                    kind.gen_js_to_bytes(name, depth + 1)
                ));
                code
            }
        }
    }

    pub fn gen_bytes_to_js(&self, depth: usize) -> String {
        match self {
            Kind::Unit => "{}".to_string(),
            Kind::I8 => format!("_b2.rI8()"),
            Kind::I16 => format!("_b2.rI16()"),
            Kind::I32 => format!("_b2.rI32()"),
            Kind::I64 => format!("_b2.rI64()"),
            Kind::I128 => format!("_b2.rI128()"),
            Kind::U8 => format!("_b2.rU8()"),
            Kind::U16 => format!("_b2.rU16()"),
            Kind::U32 => format!("_b2.rU32()"),
            Kind::U64 => format!("_b2.rU64()"),
            Kind::U128 => format!("_b2.rU128()"),
            Kind::F32 => format!("_b2.rF32()"),
            Kind::F64 => format!("_b2.rF64()"),
            Kind::Bool => format!("_b2.rB()"),
            Kind::String => format!("_b2.rS()"),
            Kind::Vec(kind) => {
                let mut code = String::new();
                code.push_str(&format!(
                    "Array.from({{length:_b2.rU32()}},()=>({}))",
                    kind.gen_bytes_to_js(depth + 1)
                ));
                code
            }
            Kind::Enum { variants } => {
                let mut code = String::from("(() => {");
                code.push_str("let _type = _b2.rU32();");
                code.push_str("switch(_type) {");

                for (i, variant) in variants.iter().enumerate() {
                    code.push_str(&format!(
                        "case {}: return {};",
                        i,
                        match variant {
                            // 结构体变体：Enum::Variant { a, b }
                            Kind::Struct { fields } => {
                                let field_code = fields
                                    .iter()
                                    .map(|f| {
                                        format!("{}: {}", f.name, f.kind.gen_bytes_to_js(depth + 1))
                                    })
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                format!("({{ _type: {}, {} }})", i, field_code)
                            }

                            // 元组变体：Enum::Variant(i32, String, ...)
                            Kind::Tuple(elements) => {
                                if elements.is_empty() {
                                    // 空元组
                                    format!("({{ _type: {} }})", i)
                                } else if elements.len() == 1 {
                                    // 单值元组 => { _type: X, value: T }
                                    format!(
                                        "({{ _type: {}, value: {} }})",
                                        i,
                                        elements[0].gen_bytes_to_js(depth + 1)
                                    )
                                } else {
                                    // 多值元组 => { _type: X, values: [T1, T2, ...] }
                                    let values = elements
                                        .iter()
                                        .map(|e| e.gen_bytes_to_js(depth + 1))
                                        .collect::<Vec<_>>()
                                        .join(", ");
                                    format!("({{ _type: {}, values: [{}] }})", i, values)
                                }
                            }

                            // 单值或空值：Enum::Variant 或 Enum::Variant(Unit)
                            _ => format!("({{ _type: {} }})", i),
                        }
                    ));
                }

                code.push_str("}})()");
                code
            }
            Kind::Struct { fields } => {
                let mut code = String::new();
                code.push_str(
                    &fields
                        .iter()
                        .map(|f| format!("{}:{}", f.name, f.kind.gen_bytes_to_js(depth + 1)))
                        .collect::<Vec<String>>()
                        .join(","),
                );
                format!("{{{}}}", code)
            }
            Kind::Tuple(elements) => {
                if elements.is_empty() {
                    "([])".to_string()
                } else {
                    let parts = elements
                        .iter()
                        .map(|e| e.gen_bytes_to_js(depth + 1))
                        .collect::<Vec<_>>()
                        .join(",");
                    format!("[{}]", parts)
                }
            }
            Kind::Nullable(kind) => {
                let mut code = String::new();
                code.push_str(&format!(
                    "_b2.rU8()===1?{}:null",
                    kind.gen_bytes_to_js(depth + 1)
                ));
                code
            }
        }
    }

    pub fn gen_js_type(&self) -> String {
        match self {
            Kind::Unit => "undefined".to_string(),
            Kind::I8 => "number".to_string(),
            Kind::I16 => "number".to_string(),
            Kind::I32 => "number".to_string(),
            Kind::I64 => "number".to_string(),
            Kind::I128 => "number".to_string(),
            Kind::U8 => "number".to_string(),
            Kind::U16 => "number".to_string(),
            Kind::U32 => "number".to_string(),
            Kind::U64 => "number".to_string(),
            Kind::U128 => "number".to_string(),
            Kind::F32 => "number".to_string(),
            Kind::F64 => "number".to_string(),
            Kind::Bool => "boolean".to_string(),
            Kind::String => "string".to_string(),
            Kind::Vec(kind) => format!("Array<{}>", kind.gen_js_type()),
            Kind::Enum { variants } => {
                let mut code = Vec::new();
                for (index, variant) in variants.iter().enumerate() {
                    let variant_type = match variant {
                        Kind::Struct { fields } => {
                            let mut fields_with_type = vec![format!("_type:{}", index)];
                            for field in fields {
                                fields_with_type.push(format!(
                                    "{}:{}",
                                    field.name,
                                    field.kind.gen_js_type()
                                ));
                            }
                            format!("{{{}}}", fields_with_type.join(","))
                        }
                        _ => {
                            format!("{{_type:{}}}", index)
                        }
                    };
                    code.push(variant_type);
                }
                code.join("|")
            }
            Kind::Struct { fields } => {
                let mut code = String::new();
                code.push_str(
                    &fields
                        .iter()
                        .map(|f| format!("{}:{}", f.name, f.kind.gen_js_type()))
                        .collect::<Vec<String>>()
                        .join(","),
                );
                format!("{{{}}}", code)
            }
            Kind::Tuple(elements) => {
                if elements.is_empty() {
                    "[]".to_string()
                } else {
                    format!(
                        "[{}]",
                        elements
                            .iter()
                            .map(|e| e.gen_js_type())
                            .collect::<Vec<_>>()
                            .join(",")
                    )
                }
            }
            Kind::Nullable(kind) => format!("{}|null", kind.gen_js_type()),
        }
    }

    pub fn gen_js_validate(&self, name: &str, tag: Option<&Tag>, depth: usize) -> String {
        let mut validations = Vec::new();

        // 类型特定的验证规则
        match self {
            Kind::I8
            | Kind::I16
            | Kind::I32
            | Kind::I64
            | Kind::I128
            | Kind::U8
            | Kind::U16
            | Kind::U32
            | Kind::U64
            | Kind::U128
            | Kind::F32
            | Kind::F64 => {
                if let Some(tag) = tag {
                    if let Some(ref required_msg) = tag.required {
                        validations.push(format!(
                            "if({0}===undefined||{0}===null||{0}===NaN||{0}===0) throw new AFastValidateError({1:?});",
                            name, required_msg
                        ));
                    }
                    if let Some((min_val, min_msg)) = &tag.min {
                        validations.push(format!(
                            "if({0}<{1})throw new AFastValidateError({2:?});",
                            name, min_val, min_msg
                        ));
                    }
                    if let Some((max_val, max_msg)) = &tag.max {
                        validations.push(format!(
                            "if({0}>{1})throw new AFastValidateError({2:?});",
                            name, max_val, max_msg
                        ));
                    }
                }
            }
            Kind::String => {
                if let Some(tag) = tag {
                    if let Some(ref required_msg) = tag.required {
                        validations.push(format!(
                            "if({0}===undefined||{0}===null||{0}==='') throw new AFastValidateError({1:?});",
                            name, required_msg
                        ));
                    }
                    if let Some((min_val, min_msg)) = &tag.min {
                        validations.push(format!(
                            "if({0}.length<{1})throw new AFastValidateError({2:?});",
                            name, min_val, min_msg
                        ));
                    }
                    if let Some((max_val, max_msg)) = &tag.max {
                        validations.push(format!(
                            "if({0}.length>{1})throw new AFastValidateError({2:?});",
                            name, max_val, max_msg
                        ));
                    }
                }
            }
            Kind::Vec(inner_kind) => {
                if let Some(tag) = tag {
                    if let Some(ref required_msg) = tag.required {
                        validations.push(format!(
                            "if({0}===undefined||{0}===null||{0}.length===0) throw new AFastValidateError({1:?});",
                            name, required_msg
                        ));
                    }
                    if let Some((min_val, min_msg)) = &tag.min {
                        validations.push(format!(
                            "if({0}.length<{1})throw new AFastValidateError({2:?});",
                            name, min_val, min_msg
                        ));
                    }
                    if let Some((max_val, max_msg)) = &tag.max {
                        validations.push(format!(
                            "if({0}.length>{1})throw new AFastValidateError({2:?});",
                            name, max_val, max_msg
                        ));
                    }
                }
                let inner_validation =
                    inner_kind.gen_js_validate(&format!("_item{}", depth), tag, depth + 1);
                if !inner_validation.is_empty() {
                    validations.push(format!(
                        "for(let _item{} of {}){{{}}}",
                        depth, name, inner_validation
                    ));
                }
            }
            Kind::Nullable(inner_kind) => {
                let inner_validation = inner_kind.gen_js_validate(name, tag, depth + 1);
                if !inner_validation.is_empty() {
                    validations.push(format!(
                        "if({0}!==null&&{0}!==undefined){{{1}}}",
                        name, inner_validation
                    ));
                }
            }
            Kind::Struct { fields } => {
                for field in fields {
                    let field_name = format!("{}.{}", name, field.name);
                    let field_validation =
                        field
                            .kind
                            .gen_js_validate(&field_name, field.tag.as_ref(), depth + 1);
                    if !field_validation.is_empty() {
                        validations.push(field_validation);
                    }
                }
            }
            Kind::Enum { variants } => {
                validations.push(format!(
                    "if({0}._type===undefined)throw new AFastValidateError('Missing _type field in {0}');",
                    name
                ));
                validations.push(format!(
                    "if(![{0}].includes({1}._type))throw new AFastValidateError(`Invalid _type field in {1}: ${{{1}._type}} not in [{0}]`);",
                    variants.iter().enumerate().map(|(i,_)| i.to_string()).collect::<Vec<String>>().join(","),
                    name,
                ));

                let mut has_variant_validation = false;
                let mut cases = Vec::new();

                for (i, variant) in variants.iter().enumerate() {
                    let variant_validation = match variant {
                        Kind::Struct { fields } => {
                            let mut field_validations = Vec::new();
                            for field in fields {
                                let field_name = format!("{}.{}", name, field.name);
                                let field_validation = field.kind.gen_js_validate(
                                    &field_name,
                                    field.tag.as_ref(),
                                    depth + 1,
                                );
                                if !field_validation.is_empty() {
                                    field_validations.push(field_validation);
                                }
                            }
                            field_validations.join("")
                        }
                        _ => String::new(),
                    };

                    if !variant_validation.is_empty() {
                        has_variant_validation = true;
                        cases.push(format!("case {}:{{{}}}break;", i, variant_validation));
                    } else {
                        cases.push(format!("case {}:break;", i));
                    }
                }

                if has_variant_validation {
                    validations.push(format!(
                        "if({0}._type===undefined)throw new AFastValidateError('Missing _type field in {0}');",
                        name
                    ));

                    cases.push(format!(
                        "default:throw new AFastValidateError(`Invalid _type field in {0}: ${{{0}._type}}`);",
                        name
                    ));
                    validations.push(format!("switch({}._type){{{}}}", name, cases.join("")));
                }
            }
            Kind::Tuple(elements) => {
                // 验证每个元素
                for (i, elem) in elements.iter().enumerate() {
                    let item_name = format!("{}[{}]", name, i);
                    let item_validation = elem.gen_js_validate(&item_name, tag, depth + 1);
                    if !item_validation.is_empty() {
                        validations.push(item_validation);
                    }
                }
            }
            _ => {}
        }

        validations.join("")
    }
}

pub fn simple_js_builder<T, H>(handlers: &Vec<HandlerGeneric<T, H>>) -> String
where
    T: Clone + Send + Sync + 'static,
    H: AFastKind + AFastData,
{
    // Group handlers by namespace
    let mut groups: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for handler in handlers {
        let namespace_key = handler.namespace.join(".");
        groups
            .entry(namespace_key)
            .or_insert_with(Vec::new)
            .push(handler.js.clone());
    }

    build_js_object(&groups, "", true)
}

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

pub fn gen_js_code<T, H>(util: bool, index: usize, handlers: &Vec<HandlerGeneric<T, H>>) -> String
where
    T: Clone + Send + Sync + 'static,
    H: AFastKind + AFastData,
{
    if util {
        let header_type = H::kind().gen_js_type().to_string();
        let js_content = format!(
            "{}\n\nexport class AFastClient {{\noffset={};\n/**\n * Create client\n * @param {{{{header:()=>Promise<{}>,hook:(data:{})=>Promise<any>,call:(buf:Uint8Array)=>Promise<Uint8Array>,}}}} options\n */\nconstructor(options){{this._options=options;if(!options.header){{throw new Error('header is required');}};this._header=options.header;this._hook=options.hook;if(!options.call){{throw new Error('call is required');}};this._call=options.call}}\n{}\n}}",
            JS,
            index,
            header_type,
            header_type,
            simple_js_builder(&handlers)
        );
        js_content
    } else {
        let header_type = H::kind().gen_js_type().to_string();
        let js_content = format!(
            "export class AFastClient {{\noffset={};\n/**\n * Create client\n * @param {{{{header:()=>Promise<{}>,hook:(data:{})=>Promise<any>,call:(buf:Uint8Array)=>Promise<Uint8Array>,}}}} options\n */\nconstructor(options){{this._options=options;if(!options.header){{throw new Error('header is required');}};this._header=options.header;this._hook=options.hook;if(!options.call){{throw new Error('call is required');}};this._call=options.call}}\n{}\n}}",
            index,
            header_type,
            header_type,
            simple_js_builder(&handlers)
        );
        js_content
    }
}
