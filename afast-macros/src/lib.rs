//! # AFast
//! 
//! **AFast** is a high-performance asynchronous Rust backend framework designed
//! to simplify building networked applications. It supports multiple protocols
//! via feature flags and provides automatic code generation for clients
//! (TypeScript and JavaScript), API documentation, and field validation.
//! 
//! ## Instructions
//! 
//! ### Supported Protocol Features
//! 
//! You can enable the following features in your `Cargo.toml`:
//! 
//! - `http` - enable HTTP support
//!   - `/api` - HTTP API endpoint
//!   - `/js` - JavaScript client
//!   - `/ts` - TypeScript client
//! - `ws` - enable WebSocket support
//!   - `/ws` - WebSocket endpoint
//! - `tcp` - enable TCP support
//! 
//! **Note on TCP usage:**  
//! 
//! If the `tcp` feature is enabled, the `AFast::serve` method takes two arguments:
//! 
//! 1. The TCP address to listen on (`"127.0.0.1:8080"`).  
//! 2. The HTTP/WS address (`"127.0.0.1:8081"`) for web clients and generated JS/TS clients.
//! 
//! This allows you to run TCP and HTTP/WS servers simultaneously in the same application.
//! 
//! ### Key Features
//! 
//! - **`handler` Macro**: Declare HTTP endpoints with minimal boilerplate
//!   - Automatic TypeScript/JavaScript client generation
//!   - Namespace support for organized API structure (`ns("api.v1.user")`)
//!   - Middleware chaining for authentication/validation (`mws("auth")`)
//!   - Descriptive API documentation generation (`desc("Get user info")`)
//! - Automatic field validation with custom rules
//! - Async handler functions with state management
//! - Flexible multi-protocol support: HTTP, WS, TCP
//! 
//! #### Handler Macro Overview
//! 
//! The `#[handler]` attribute macro transforms async functions into full-featured API endpoints:
//! 
//! ```rust
//! #[handler(desc("Get user information"), ns("api.v1.user"), mws("auth"))]
//! async fn get_user(state: Arc<Mutex<String>>, header: Header, req: Request) -> Result<Response, Error> {
//!     // Your business logic
//! }
//! ```
//! 
//! **Macro Parameters:**
//! 
//! - `desc("description")` - API description for documentation
//! - `ns("api.v1.user")` - Namespace for nested JS client generation
//! - `mws("auth,validation")` - Middleware chain for pre-processing
//! 
//! **Generated Output:**
//! 
//! - Type-safe HTTP endpoints
//! - Nested JavaScript client structure
//! - TypeScript type definitions  
//! - OpenAPI documentation
//! 
//! ### Upcoming Features / Development Plan
//! 
//! - Nested structure validation for complex types
//! - Enable or disable js / ts / document by feature flags
//! - Add command for generating client code
//! - Generate client code for additional languages: Java, Kotlin, C#, Rust, etc.
//! - Improved code generation templates for easier integration
//! - Enhanced error handling and validation reporting
//! 
//! ## Example
//! 
//! ```rust
//! use std::sync::{Arc, Mutex};
//! 
//! use afast::{AFast, AFastData, Error, handler, register};
//! 
//! #[derive(Debug, Clone, AFastData)]
//! enum Sex {
//!     Male { id: i64 },
//!     Female { name: String },
//! }
//! 
//! #[derive(Debug, Clone, AFastData)]
//! struct Request {
//!     id: i64,
//!     name: String,
//!     #[validate(
//!         required("name is required"),
//!         min(1, "name must be at least 1 character long"),
//!         max(100, "name must be at most 10 characters long")
//!     )]
//!     age: u32,
//!     hobbies: Vec<Hobby>,
//!     tags: Vec<String>,
//!     gender: Option<bool>,
//!     sex: Sex,
//! }
//! 
//! #[derive(Debug, Clone, AFastData)]
//! struct Hobby {
//!     id: i64,
//!     name: String,
//! }
//! 
//! #[derive(Debug, AFastData)]
//! pub struct Response {
//!     sex: Sex,
//!     id: i64,
//!     name: String,
//!     age: u32,
//!     hobbies: Vec<Hobby>,
//!     tags: Vec<String>,
//!     gender: Option<bool>,
//! }
//! 
//! #[handler(desc("Get user information"), ns("api.user"))]
//! async fn get_user(
//!     _state: Arc<Mutex<String>>,
//!     header: Header,
//!     req: Request,
//! ) -> Result<Response, Error> {
//!     Ok(Response {
//!         id: req.id,
//!         name: req.name.clone(),
//!         age: req.age,
//!         hobbies: req.hobbies.clone(),
//!         tags: req.tags.clone(),
//!         gender: req.gender,
//!         sex: req.sex.clone(),
//!     })
//! }
//! 
//! async fn auth(_state: Arc<Mutex<String>>, header: Header) -> Result<(), Error> {
//!     println!("Token: {:?}", header);
//!     Ok(())
//! }
//! 
//! #[derive(Debug, AFastData)]
//! struct Req2 {
//!     id: i64,
//! }
//! 
//! #[derive(Debug, AFastData)]
//! struct Resp2 {
//!     id: i64,
//!     name: String,
//! }
//! 
//! #[handler(desc("Get user by id"), mws("auth"), ns("api"))]
//! async fn get_id(_state: Arc<Mutex<String>>, header: Header, req: Req2) -> Result<Resp2, Error> {
//!     Ok(Resp2 {
//!         id: req.id,
//!         name: "John".to_string(),
//!     })
//! }
//! 
//! #[derive(Debug, Clone, AFastData)]
//! struct Header {
//!     id: u32,
//! }
//! 
//! #[tokio::main]
//! async fn main() {
//!     let state = Arc::new(Mutex::new("".to_string()));
//! 
//!     let server = AFast::<Mutex<String>, Header>::new(state, register! { get_user, get_id })
//!         .set_js(true) // Auto generate JS client
//!         .set_doc(true); // Auto generate documentation
//! 
//!     server
//!         .serve(&"127.0.0.1:8080", &"127.0.0.1:8081")
//!         .await
//!         .unwrap();
//! }
//! ```
//! 

use proc_macro::TokenStream;

mod atd;
mod deserialize;
mod serialize;

#[proc_macro_attribute]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    atd::handler(attr, item)
}

#[proc_macro]
pub fn register(input: TokenStream) -> TokenStream {
    atd::register(input)
}

use proc_macro2::TokenStream as TS;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Error, LitInt, LitStr, Meta, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

#[proc_macro_derive(AFastData, attributes(validate, afast))]
pub fn serialize_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (
        serialize_code,
        deserialize_code,
        serialize_js_code,
        serialize_js_validate,
        serialize_js_type,
        deserialize_js_code,
        validate_code,
    ) = match &input.data {
        Data::Struct(data) => match serialize_struct(&name, data) {
            Ok(ts) => ts,
            Err(e) => return e.to_compile_error().into(),
        },
        Data::Enum(data) => match serialize_enum(&name, data) {
            Ok(ts) => ts,
            Err(e) => return e.to_compile_error().into(),
        },
        Data::Union(_data) => {
            panic!("unions are not supported yet");
        }
    };

    let expanded = quote! {
        impl ::afast::AFastData for #name {
            fn to_bytes(&self) -> Vec<u8> {
                let mut buf = Vec::new();
                #(#serialize_code)*
                buf
            }

            fn from_bytes(buf: &[u8]) -> Result<(Self, usize), ::afast::Error> {
                let _length = buf.len();
                let mut _offset = 0;
                #(#deserialize_code)*
            }

            fn to_js(prefix: &str) -> String {
                let mut code = String::new();
                #(#serialize_js_code)*
                code
            }

            fn to_ts(prefix: &str) -> String {
                let mut code = String::new();
                #(#serialize_js_code)*
                code
            }

            fn to_js_type(prefix: &str) -> String {
                let mut code = String::new();
                #(#serialize_js_type)*
                code
            }

            fn to_js_validate(prefix: &str) -> String {
                let mut code = String::new();
                #(#serialize_js_validate)*
                code
            }

            fn from_js(parent: &str) -> String {
                let mut code = String::new();
                code.push_str("{");
                #(#deserialize_js_code)*
                code.push_str("}");
                code
            }

            fn validate(&self) -> Result<(), Vec<&'static str>> {
                #(#validate_code)*
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}

fn serialize_struct(
    _name: &syn::Ident,
    data: &syn::DataStruct,
) -> Result<
    (
        Vec<TS>,
        Vec<TS>,
        Vec<TS>,
        Vec<TS>,
        Vec<TS>,
        Vec<TS>,
        Vec<TS>,
    ),
    Error,
> {
    let mut serialize_code = Vec::new();
    let mut deserialize_code = Vec::new();
    let mut serialize_js_code = Vec::new();
    let mut serialize_js_type = Vec::new();
    let mut serialize_js_validate = Vec::new();
    let mut deserialize_js_code = Vec::new();
    let mut valideate_code = Vec::new();
    let mut field_inits = Vec::new();

    serialize_js_type.push(quote! {code.push_str("{");});

    match &data.fields {
        syn::Fields::Named(fields_named) => {
            for field in fields_named.named.iter() {
                let ident = &field.ident;
                field_inits.push(ident);
                let ty = &field.ty;
                let typ = quote!(#ty).to_string();
                let tag = parse_tags(&field.attrs)?;
                serialize_code.push(serialize::gen_struct_code::gen_serialize_code(
                    &ty,
                    quote!(self.#ident),
                    0,
                ));
                deserialize_code.push(deserialize::gen_struct_code::gen_deserialize_code(
                    &ty,
                    quote!(#ident),
                    0,
                ));
                serialize_js_code.push(serialize::gen_js_code::gen_js_code(
                    &typ,
                    &quote!(#ident),
                    &quote!(request),
                    0,
                    &ty,
                ));
                let field_name = ident.clone().unwrap().to_string();
                serialize_js_type.push(quote! {
                    code.push_str(&format!("{}:", #field_name));
                });
                serialize_js_type.push(serialize::gen_js_type::gen_js_type(
                    &typ,
                    &quote!(#ident),
                    &quote!(request),
                    0,
                    &ty,
                ));
                serialize_js_type.push(quote! {
                    code.push_str(";");
                });
                deserialize_js_code.push(quote! {
                    code.push_str(&format!("{}:", #field_name));
                });
                deserialize_js_code.push(deserialize::gen_js_code::gen_js_code(
                    &typ,
                    &quote!(#ident),
                    &quote!(request),
                    0,
                    &ty,
                ));
                deserialize_js_code.push(quote! {
                    code.push_str(",");
                });
                let field_name = ident.clone().unwrap().to_string();
                let typ = typ.replace(" ", "");
                if typ.starts_with("Vec") {
                    if let Some(msg) = tag.required {
                        valideate_code.push(quote! {
                            if self.#ident.is_empty() {
                                return Err(vec![#msg]);
                            }
                        });
                        serialize_js_validate.push(quote! {
                            code.push_str(&format!("if ({}.{}.length === 0) throw new AFastValidateError('{}');", prefix, #field_name, #msg));
                        });
                    }
                    if let Some((min, msg)) = tag.min {
                        valideate_code.push(quote! {
                            if self.#ident.len() < #min as usize {
                                return Err(vec![#msg]);
                            }
                        });
                        serialize_js_validate.push(quote! {
                            code.push_str(&format!("if ({}.{}.length < {}) throw new AFastValidateError('{}');", prefix, #field_name, #min, #msg));
                        });
                    }
                    if let Some((max, msg)) = tag.max {
                        valideate_code.push(quote! {
                            if self.#ident.len() > #max as usize {
                                return Err(vec![#msg]);
                            }
                        });
                        serialize_js_validate.push(quote! {
                            code.push_str(&format!("if ({}.{}.length > {}) throw new AFastValidateError('{}');", prefix, #field_name, #max, #msg));
                        });
                    }
                } else if typ.starts_with("Option") {
                } else {
                    match typ.as_str() {
                        "String" => {
                            if let Some(msg) = tag.required {
                                valideate_code.push(quote! {
                                    if self.#ident.is_empty() {
                                        return Err(vec![#msg]);
                                    }
                                });
                                serialize_js_validate.push(quote! {
                                    code.push_str(&format!("if ({}.{}.length === 0) throw new AFastValidateError('{}');", prefix, #field_name, #msg));
                                });
                            }
                        }
                        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32"
                        | "u64" | "u128" | "usize" | "f32" | "f64" => {
                            if let Some(msg) = tag.required {
                                valideate_code.push(quote! {
                                    if self.#ident == 0 {
                                        return Err(vec![#msg]);
                                    }
                                });
                                serialize_js_validate.push(quote! {
                                    code.push_str(&format!("if ({}.{} === 0) throw new AFastValidateError('{}');", prefix, #field_name, #msg));
                                });
                            }
                            if let Some((min, msg)) = tag.min {
                                valideate_code.push(quote! {
                                    if self.#ident < #min as #ty {
                                        return Err(vec![#msg]);
                                    }
                                });
                                serialize_js_validate.push(quote! {
                                    code.push_str(&format!("if ({}.{} < {}) throw new AFastValidateError('{}');", prefix, #field_name, #min, #msg));
                                });
                            }
                            if let Some((max, msg)) = tag.max {
                                valideate_code.push(quote! {
                                    if self.#ident > #max as #ty {
                                        return Err(vec![#msg]);
                                    }
                                });
                                serialize_js_validate.push(quote! {
                                    code.push_str(&format!("if ({}.{} > {}) throw new AFastValidateError('{}');", prefix, #field_name, #max, #msg));
                                });
                            }
                        }
                        _ => {
                            valideate_code.push(quote! {
                                self.#ident.validate()?;
                            });
                        }
                    }
                }
            }
        }
        syn::Fields::Unnamed(_fields_unnamed) => {}
        syn::Fields::Unit => {}
    }

    serialize_js_type.push(quote! {code.push_str("}");});

    deserialize_code.push(quote! {
        Ok((Self { #(#field_inits:#field_inits,)* }, _offset))
    });

    Ok((
        serialize_code,
        deserialize_code,
        serialize_js_code,
        serialize_js_validate,
        serialize_js_type,
        deserialize_js_code,
        valideate_code,
    ))
}

fn serialize_enum(
    _name: &syn::Ident,
    data: &syn::DataEnum,
) -> Result<
    (
        Vec<TS>,
        Vec<TS>,
        Vec<TS>,
        Vec<TS>,
        Vec<TS>,
        Vec<TS>,
        Vec<TS>,
    ),
    Error,
> {
    let mut serialize_code = Vec::new();
    let mut deserialize_code = Vec::new();
    let mut serialize_js_code = Vec::new();
    let mut serialize_js_type = Vec::new();
    let mut deserialize_js_code = Vec::new();
    let mut valideate_code = Vec::new();

    let count = data.variants.len();

    for (idx, variant) in data.variants.iter().enumerate() {
        let index = idx as u32;
        let variant_ident = &variant.ident;
        match &variant.fields {
            syn::Fields::Named(fields_named) => {
                let mut field_idents = Vec::new();
                let mut ser_fields = Vec::new();
                let mut deser_fields = Vec::new();
                let mut ser_js = Vec::new();
                let mut ser_js_type = Vec::new();
                let mut deser_js = Vec::new();
                for field in fields_named.named.iter() {
                    ser_js_type.push(quote! {
                        code.push_str(&format!("_type:{};", #idx));
                    });
                    let ident = &field.ident;
                    field_idents.push(ident);
                    let ty = &field.ty;
                    let typ = quote!(#ty).to_string();
                    let tag = parse_tags(&field.attrs)?;
                    ser_fields.push(serialize::gen_struct_code::gen_serialize_code(
                        &ty,
                        quote!(#ident),
                        0,
                    ));
                    deser_fields.push(deserialize::gen_struct_code::gen_deserialize_code(
                        &ty,
                        quote!(#ident),
                        0,
                    ));
                    ser_js.push(serialize::gen_js_code::gen_js_code(
                        &typ,
                        &quote!(#ident),
                        &quote!(request),
                        0,
                        &ty,
                    ));
                    let field_name = ident.clone().unwrap().to_string();
                    ser_js_type.push(quote! {
                        code.push_str(&format!("{}:", #field_name));
                    });
                    ser_js_type.push(serialize::gen_js_type::gen_js_type(
                        &typ,
                        &quote!(#ident),
                        &quote!(request),
                        0,
                        &ty,
                    ));
                    ser_js_type.push(quote! {
                        code.push_str(";");
                    });
                    deser_js.push(quote! {
                        code.push_str(&format!("{}:", #field_name));
                    });
                    deser_js.push(deserialize::gen_js_code::gen_js_code(
                        &typ,
                        &quote!(#ident),
                        &quote!(request),
                        0,
                        &ty,
                    ));
                    deser_js.push(quote! {
                        code.push_str(",");
                    });
                    let typ = typ.replace(" ", "");
                    if typ.starts_with("Vec") {
                        if let Some(msg) = tag.required {
                            valideate_code.push(quote! {
                                if self.#ident.is_empty() {
                                    return Err(vec![#msg]);
                                }
                            });
                        }
                        if let Some((min, msg)) = tag.min {
                            valideate_code.push(quote! {
                                if self.#ident.len() < #min as usize {
                                    return Err(vec![#msg]);
                                }
                            });
                        }
                        if let Some((max, msg)) = tag.max {
                            valideate_code.push(quote! {
                                if self.#ident.len() > #max as usize {
                                    return Err(vec![#msg]);
                                }
                            });
                        }
                    } else if typ.starts_with("Option") {
                    } else {
                        match typ.as_str() {
                            "String" => {
                                if let Some(msg) = tag.required {
                                    valideate_code.push(quote! {
                                        if self.#ident.is_empty() {
                                            return Err(vec![#msg]);
                                        }
                                    });
                                }
                            }
                            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16"
                            | "u32" | "u64" | "u128" | "usize" | "f32" | "f64" => {
                                if let Some(msg) = tag.required {
                                    valideate_code.push(quote! {
                                        if self.#ident == 0 {
                                            return Err(vec![#msg]);
                                        }
                                    });
                                }
                                if let Some((min, msg)) = tag.min {
                                    valideate_code.push(quote! {
                                        if self.#ident < #min as #ty {
                                            return Err(vec![#msg]);
                                        }
                                    });
                                }
                                if let Some((max, msg)) = tag.max {
                                    valideate_code.push(quote! {
                                        if self.#ident > #max as #ty {
                                            return Err(vec![#msg]);
                                        }
                                    });
                                }
                            }
                            "Option<u32>" => {}
                            _ => {
                                valideate_code.push(quote! {
                                    self.#ident.validate()?;
                                });
                            }
                        }
                    }
                }

                serialize_code.push(quote! {
                    Self::#variant_ident { #( #field_idents ),* } => {
                        buf.extend(&#index.to_be_bytes());
                        #( #ser_fields )*
                    }
                });

                serialize_js_type.push(quote! {
                    code.push_str("{");
                    #( #ser_js_type )*
                    code.push_str("}");
                });

                if idx < count - 1 {
                    serialize_js_type.push(quote! {
                        code.push_str("|");
                    });
                }

                deserialize_code.push(quote! {
                    #index => {
                        #( #deser_fields )*
                        Ok((Self::#variant_ident { #( #field_idents ),* }, _offset))
                    }
                });

                serialize_js_code.push(quote! {
                    code.push_str(&format!("case {}:", #index));
                    #( #ser_js )*
                    code.push_str("break;");
                });

                deserialize_js_code.push(quote! {
                    code.push_str(&format!("case {}: return {{", #index));
                    #( #deser_js )*
                    code.push_str("};");
                });
            }
            syn::Fields::Unnamed(_fields_unnamed) => {}
            syn::Fields::Unit => {}
        }
    }

    Ok((
        vec![quote! {
            match self {
                #( #serialize_code )*
            }
        }],
        vec![quote! {
            let mut _tag = [0; 4];
            _tag.copy_from_slice(&buf[_offset.._offset+4]);
            _offset += 4;
            match u32::from_be_bytes(_tag) {
                #( #deserialize_code )*
                _ => Err(::afast::Error::DecodeError)
            }
        }],
        vec![quote! {
            code.push_str(&format!("_b1.pU32({}._type);switch ({}._type) {{", prefix, prefix));
            #( #serialize_js_code )*
            code.push_str("default:throw new Error('unknown variant');}");
        }],
        vec![],
        serialize_js_type,
        vec![quote! {
            code.push_str("...(function(){switch (_b2.rU32()) {");
            #( #deserialize_js_code )*
            code.push_str("default:throw new Error('unknown variant');}}())");
        }],
        valideate_code,
    ))
}

#[derive(Debug)]
struct Tag {
    name: Option<String>,
    description: Option<String>,
    required: Option<String>,
    min: Option<(i64, String)>,
    max: Option<(i64, String)>,
}

struct IntStrTuple {
    int: LitInt,
    _comma: Token![,],
    msg: LitStr,
}

impl Parse for IntStrTuple {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(IntStrTuple {
            int: input.parse()?,
            _comma: input.parse()?,
            msg: input.parse()?,
        })
    }
}

fn parse_tags(attrs: &Vec<Attribute>) -> Result<Tag, Error> {
    let mut tag = Tag {
        name: None,
        description: None,
        required: None,
        min: None,
        max: None,
    };
    for attr in attrs {
        if attr.path().is_ident("validate") {
            let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
            for meta in nested {
                match meta {
                    Meta::Path(_path) => {}
                    Meta::List(meta) => {
                        if meta.path.is_ident("name") {
                            let lit = meta.parse_args::<LitStr>()?;
                            tag.name = Some(lit.value());
                        } else if meta.path.is_ident("description") {
                            let lit = meta.parse_args::<LitStr>()?;
                            tag.description = Some(lit.value());
                        } else if meta.path.is_ident("required") {
                            let lit = meta.parse_args::<LitStr>()?;
                            tag.required = Some(lit.value());
                        } else if meta.path.is_ident("min") {
                            let inner = meta.parse_args::<IntStrTuple>()?;
                            let min = inner.int.base10_parse::<i64>()?;
                            let msg = inner.msg.value();
                            tag.min = Some((min, msg));
                        } else if meta.path.is_ident("max") {
                            let inner = meta.parse_args::<IntStrTuple>()?;
                            let max = inner.int.base10_parse::<i64>()?;
                            let msg = inner.msg.value();
                            tag.max = Some((max, msg));
                        }
                    }
                    Meta::NameValue(_meta) => {}
                }
            }
        }
    }
    Ok(tag)
}
