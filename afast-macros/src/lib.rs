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
//!   - `/api` - HTTP API endpoints
//!   - `/js` - JavaScript client (requires `js` feature)
//!   - `/ts` - TypeScript client (requires `js` feature)
//! - `ws` - enable WebSocket support
//!   - `/ws` - WebSocket endpoint
//! - `tcp` - enable TCP support
//! - `js` - enable JavaScript & TypeScript client generation
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
//!     let server =
//!         AFast::<Mutex<String>, Header>::new(state).service("user", register! { get_user, get_id });
//!
//!     server
//!         .serve(
//!             #[cfg(feature = "tcp")]
//!             &"127.0.0.1:8080",
//!             &"127.0.0.1:8081",
//!         )
//!         .await
//!         .unwrap();
//! }
//! ```
//!

use proc_macro::TokenStream;

mod atd;
mod deserialize;
mod handler;
mod serialize;

#[proc_macro_attribute]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    atd::handler(attr, item)
}

#[proc_macro]
pub fn register(input: TokenStream) -> TokenStream {
    atd::register(input)
}

use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, LitInt, LitStr, Meta, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned as _,
};

#[proc_macro_derive(AFastData, attributes(validate, afast))]
pub fn serialize_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut code = Vec::new();

    let (serialize_code, deserialize_code, validate_code) = match &input.data {
        Data::Struct(data) => match handler::rust::handler_struct(&name, data) {
            Ok(ts) => ts,
            Err(e) => return e.to_compile_error().into(),
        },
        Data::Enum(data) => match handler::rust::handler_enum(&name, data) {
            Ok(ts) => ts,
            Err(e) => return e.to_compile_error().into(),
        },
        Data::Union(_data) => {
            panic!("unions are not supported yet");
        }
    };

    code.push(quote! {
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

        fn validate(&self) -> Result<(), Vec<&'static str>> {
            #(#validate_code)*
            Ok(())
        }
    });

    let expanded = quote! {
        impl ::afast::AFastData for #name {
            #(#code)*
        }
    };

    TokenStream::from(expanded)
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

#[proc_macro_derive(AFastKind, attributes(validate, afast))]
pub fn derive_get_kind(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;

    let kind_impl = match generate_kind_expression(&input.data) {
        Ok(kind_impl) => kind_impl,
        Err(err) => return err.to_compile_error().into(),
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics AFastKind for #name #ty_generics #where_clause {
            fn kind() -> Kind {
                #kind_impl
            }

            fn field(name: &str) -> Field {
                Field {
                    name: name.to_string(),
                    kind: <#name as AFastKind>::kind(),
                    tag: None,
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn generate_kind_expression(data: &Data) -> Result<proc_macro2::TokenStream, syn::Error> {
    match data {
        Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields) => {
                let mut field_kinds = Vec::new();
                for f in &fields.named {
                    let field_name = f.ident.as_ref().unwrap();
                    let field_type = &f.ty;
                    let kind_expr = type_to_kind_expr(field_type);

                    let field_tag = parse_tags(&f.attrs)?;
                    let tag_expr = generate_tag_expression(&field_tag);

                    field_kinds.push(quote! {
                        Field {
                            name: stringify!(#field_name).to_string(),
                            kind: #kind_expr,
                            tag: #tag_expr,
                        }
                    });
                }

                Ok(quote! {
                    Kind::Struct {
                        fields: vec![#(#field_kinds),*]
                    }
                })
            }
            syn::Fields::Unnamed(fields) => {
                let mut field_kinds = Vec::new();
                for (i, f) in fields.unnamed.iter().enumerate() {
                    let field_type = &f.ty;
                    let kind_expr = type_to_kind_expr(field_type);

                    // 解析字段级别的 tag
                    let field_tag = parse_tags(&f.attrs)?;
                    let tag_expr = generate_tag_expression(&field_tag);

                    field_kinds.push(quote! {
                        Field {
                            name: format!("_{}", #i),
                            kind: #kind_expr,
                            tag: #tag_expr,
                        }
                    });
                }

                Ok(quote! {
                    Kind::Struct {
                        fields: vec![#(#field_kinds),*]
                    }
                })
            }
            syn::Fields::Unit => Ok(quote! {
                Kind::Struct { fields: vec![] }
            }),
        },
        Data::Enum(data_enum) => {
            let mut variant_kinds = Vec::new();
            for variant in &data_enum.variants {
                let variant_kind = match &variant.fields {
                    syn::Fields::Unit => {
                        quote! { Kind::Unit }
                    }
                    syn::Fields::Unnamed(fields) => {
                        let mut field_kinds = Vec::new();
                        for (i, f) in fields.unnamed.iter().enumerate() {
                            let field_type = &f.ty;
                            let kind_expr = type_to_kind_expr(field_type);

                            let field_tag = parse_tags(&f.attrs)?;
                            let tag_expr = generate_tag_expression(&field_tag);

                            field_kinds.push(quote! {
                                Field {
                                    name: format!("_{}", #i),
                                    kind: #kind_expr,
                                    tag: #tag_expr,
                                }
                            });
                        }

                        quote! {
                            Kind::Struct {
                                fields: vec![#(#field_kinds),*]
                            }
                        }
                    }
                    syn::Fields::Named(fields) => {
                        let mut field_kinds = Vec::new();
                        for f in &fields.named {
                            let field_name = f.ident.as_ref().unwrap();
                            let field_type = &f.ty;
                            let kind_expr = type_to_kind_expr(field_type);

                            let field_tag = parse_tags(&f.attrs)?;
                            let tag_expr = generate_tag_expression(&field_tag);

                            field_kinds.push(quote! {
                                Field {
                                    name: stringify!(#field_name).to_string(),
                                    kind: #kind_expr,
                                    tag: #tag_expr,
                                }
                            });
                        }

                        quote! {
                            Kind::Struct {
                                fields: vec![#(#field_kinds),*]
                            }
                        }
                    }
                };
                variant_kinds.push(variant_kind);
            }

            Ok(quote! {
                Kind::Enum {
                    variants: vec![#(#variant_kinds),*]
                }
            })
        }
        Data::Union(_) => Ok(quote! {
            Kind::Struct { fields: vec![] }
        }),
    }
}

fn generate_tag_expression(tag: &Tag) -> proc_macro2::TokenStream {
    let name = if let Some(ref name) = tag.name {
        quote! { Some(#name.to_string()) }
    } else {
        quote! { None }
    };

    let description = if let Some(ref desc) = tag.description {
        quote! { Some(#desc.to_string()) }
    } else {
        quote! { None }
    };

    let required = if let Some(ref req) = tag.required {
        quote! { Some(#req.to_string()) }
    } else {
        quote! { None }
    };

    let min = if let Some((min_val, msg)) = &tag.min {
        quote! { Some((#min_val, #msg.to_string())) }
    } else {
        quote! { None }
    };

    let max = if let Some((max_val, msg)) = &tag.max {
        quote! { Some((#max_val, #msg.to_string())) }
    } else {
        quote! { None }
    };

    let all_none = tag.name.is_none()
        && tag.description.is_none()
        && tag.required.is_none()
        && tag.min.is_none()
        && tag.max.is_none();

    if all_none {
        quote! { None }
    } else {
        quote! {
            Some(Tag {
                name: #name,
                description: #description,
                required: #required,
                min: #min,
                max: #max,
            })
        }
    }
}

fn parse_tags(attrs: &Vec<Attribute>) -> Result<Tag, syn::Error> {
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
                    Meta::Path(path) => {
                        return Err(syn::Error::new(
                            path.span(),
                            "Unexpected path in validate attribute. Expected `name(\"...\")`, `description(\"...\")`, etc.",
                        ));
                    }
                    Meta::List(meta) => {
                        if meta.path.is_ident("name") {
                            let lit = meta.parse_args::<syn::LitStr>()?;
                            tag.name = Some(lit.value());
                        } else if meta.path.is_ident("description") {
                            let lit = meta.parse_args::<syn::LitStr>()?;
                            tag.description = Some(lit.value());
                        } else if meta.path.is_ident("required") {
                            let lit = meta.parse_args::<syn::LitStr>()?;
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
                        } else {
                            return Err(syn::Error::new(
                                meta.path.span(),
                                format!(
                                    "Unknown validate attribute: {}. Expected one of: name, description, required, min, max",
                                    meta.path.get_ident().unwrap()
                                ),
                            ));
                        }
                    }
                    Meta::NameValue(meta) => {
                        return Err(syn::Error::new(
                            meta.span(),
                            "Unexpected name-value pair in validate attribute. Use `name = \"...\"` syntax.",
                        ));
                    }
                }
            }
        }
    }

    Ok(tag)
}

fn type_to_kind_expr(ty: &syn::Type) -> proc_macro2::TokenStream {
    match ty {
        syn::Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                let type_name = segment.ident.to_string();

                // 处理 Option<T> 和 Vec<T>
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if type_name == "Option" {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                            let inner_kind = type_to_kind_expr(inner_ty);
                            return quote! { Kind::Nullable(Box::new(#inner_kind)) };
                        }
                    } else if type_name == "Vec" {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                            let inner_kind = type_to_kind_expr(inner_ty);
                            return quote! { Kind::Vec(Box::new(#inner_kind)) };
                        }
                    }
                }

                // 基本类型处理
                match type_name.as_str() {
                    "i8" => quote! { Kind::I8 },
                    "i16" => quote! { Kind::I16 },
                    "i32" => quote! { Kind::I32 },
                    "i64" => quote! { Kind::I64 },
                    "i128" => quote! { Kind::I128 },
                    "u8" => quote! { Kind::U8 },
                    "u16" => quote! { Kind::U16 },
                    "u32" => quote! { Kind::U32 },
                    "u64" => quote! { Kind::U64 },
                    "u128" => quote! { Kind::U128 },
                    "f32" => quote! { Kind::F32 },
                    "f64" => quote! { Kind::F64 },
                    "bool" => quote! { Kind::Bool },
                    "String" => quote! { Kind::String },
                    "str" => quote! { Kind::String },
                    "()" => quote! { Kind::Unit },
                    _ => {
                        // 处理其他泛型类型
                        if let syn::PathArguments::AngleBracketed(_) = &segment.arguments {
                            quote! { <#ty as AFastKind>::kind() }
                        } else {
                            // 用户定义的非泛型类型
                            quote! { <#ty as AFastKind>::kind() }
                        }
                    }
                }
            } else {
                quote! { Kind::String }
            }
        }
        syn::Type::Tuple(type_tuple) => {
            if type_tuple.elems.is_empty() {
                quote! { Kind::Unit }
            } else {
                quote! { Kind::String }
            }
        }
        _ => quote! { Kind::String },
    }
}
