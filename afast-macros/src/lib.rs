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
//!   - `/` - Document path (feature flag `doc`)
//!   - `/api` - HTTP API endpoints
//!   - `/code/{service}/{lang}` - Client code (feature flag `js`|`ts` ...)
//!   - `/doc` - Service list (feature flag `doc`)
//!   - `/doc/{service}` - Handler defintions and documentation (feature flag `doc`)
//! - `ws` - enable WebSocket support
//!   - `/ws` - WebSocket endpoint
//! - `tcp` - enable TCP support
//! - `doc` - enable API documentation generation
//! - `js` - enable JavaScript client generation (auto enabled `code`)
//! - `ts` - enable TypeScript client generation (auto enabled `code`)
//! - `code` - enable code generation
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
//! #[handler(desc("Get user information"), ns("api.v1.user"))]
//! async fn get_user(state: String, header: Header, req: Request) -> Result<Response, Error> {
//!     // Your business logic
//! }
//! ```
//! 
//! **Macro Parameters:**
//! 
//! - `desc("description")` - API description for documentation
//! - `ns("api.v1.user")` - Namespace for nested JS client generation
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
//! use afast::{AFast, AFastData, AFastKind, Error, handler, middleware, register};
//! 
//! #[derive(Debug, Clone, AFastData, AFastKind)]
//! enum Sex {
//!     Other,
//!     Custom(#[validate(desc("Custom user sex 0"))] i32, String),
//!     Male {
//!         #[validate(desc("Male user id"))]
//!         id: i64,
//!     },
//!     Female {
//!         #[validate(desc("Female user name"))]
//!         name: String,
//!     },
//! }
//! 
//! #[derive(Debug, Clone, AFastData, AFastKind)]
//! struct Request {
//!     #[validate(desc("User ID"))]
//!     id: i64,
//!     #[validate(desc("User name"))]
//!     name: String,
//!     #[validate(
//!         desc("User age"),
//!         required("age is required"),
//!         min(1, "age must be at least 1"),
//!         max(256, "age must be at most 256")
//!     )]
//!     age: u32,
//!     #[validate(desc("User hobbies"))]
//!     hobbies: Vec<Hobby>,
//!     #[validate(desc("User tags"))]
//!     tags: Vec<String>,
//!     #[validate(desc("User gender"))]
//!     gender: Option<bool>,
//!     #[validate(desc("User sex"))]
//!     sex: Sex,
//! }
//! 
//! #[derive(Debug, Clone, AFastData, AFastKind)]
//! struct Hobby {
//!     id: i64,
//!     name: String,
//! }
//! 
//! #[derive(Debug, AFastData, AFastKind)]
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
//! async fn get_user(_state: String, _header: Header, req: Request) -> Result<Response, Error> {
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
//! #[derive(Debug, AFastData, AFastKind)]
//! struct Req2 {
//!     id: i64,
//! }
//! 
//! #[derive(Debug, AFastData, AFastKind)]
//! struct Resp2 {
//!     id: i64,
//!     name: String,
//! }
//! 
//! #[handler(desc("Get user by id"), ns("api"))]
//! async fn get_id(_state: String, _header: Header, req: Req2) -> Result<Resp2, Error> {
//!     Ok(Resp2 {
//!         id: req.id,
//!         name: "John".to_string(),
//!     })
//! }
//! 
//! #[derive(Debug, Clone, AFastData, AFastKind)]
//! struct Header {
//!     token: String,
//! }
//! 
//! #[middleware]
//! async fn auth(_state: String, header: Header) -> Result<Header, Error> {
//!     println!("Token: {:?}", header);
//!     Ok(header)
//! }
//! 
//! #[tokio::main]
//! async fn main() {
//!     let state = "".to_string();
//! 
//!     let server = AFast::<String, Header>::new(state)
//!         .service("user", "User service", register! { get_user, get_id })
//!         .middleware(auth);
//! 
//!     server
//!         .serve(
//!             #[cfg(feature = "tcp")]
//!             &"127.0.0.1:8080",
//!             #[cfg(any(feature = "http", feature = "ws"))]
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

use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, FnArg, ItemFn, LitInt, LitStr, Meta, PatType, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned as _,
};

/// Attribute macro to define an API handler.
#[proc_macro_attribute]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    atd::handler(attr, item)
}

/// Macro to register multiple handlers.
#[proc_macro]
pub fn register(input: TokenStream) -> TokenStream {
    atd::register(input)
}

#[proc_macro_attribute]
pub fn middleware(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let ident = &input.sig.ident;
    let vis = &input.vis;
    let block = &input.block;
    let sig = &input.sig;
    let mut state_ty = None;
    let mut header_ty = None;
    for (i, arg) in sig.inputs.iter().enumerate() {
        if let FnArg::Typed(PatType { ty, .. }) = arg {
            match i {
                0 => state_ty = Some(ty.clone()),
                1 => header_ty = Some(ty.clone()),
                _ => {}
            }
        }
    }

    let state_ty = state_ty.expect("Expected first parameter to be state");
    let header_ty = header_ty.expect("Expected second parameter to be header");

    TokenStream::from(quote! {
        #vis fn #ident() -> Box<afast::Middleware<#state_ty, #header_ty>> {
            Box::new(|state, header| Box::pin(async move {
                #block
            }))
        }
    })
}

/// Derive macro to generate serialization, deserialization, and validation implementations.
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
        Data::Union(_data) => panic!("unions are not supported yet"),
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

/// Derive macro to generate type metadata (kind information).
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
            fn kind() -> afast::Kind {
                #kind_impl
            }

            fn field(name: &str) -> afast::Field {
                afast::Field { name: name.to_string(), kind: <#name as AFastKind>::kind(), tag: None }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Generates `Kind` expression for struct/enum data.
fn generate_kind_expression(data: &Data) -> Result<proc_macro2::TokenStream, syn::Error> {
    match data {
        Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields) => {
                let mut field_kinds = Vec::new();
                for f in &fields.named {
                    if parse_ignore(&f.attrs)? {
                        continue;
                    }
                    let field_name = f.ident.as_ref().unwrap();
                    let field_type = &f.ty;
                    let kind_expr = type_to_kind_expr(field_type);
                    let field_tag = parse_tags(&f.attrs)?;
                    let tag_expr = generate_tag_expression(&field_tag);

                    field_kinds.push(quote! {
                        afast::Field { name: stringify!(#field_name).to_string(), kind: #kind_expr, tag: #tag_expr }
                    });
                }

                Ok(quote! { afast::Kind::Struct { fields: vec![#(#field_kinds),*] } })
            }
            syn::Fields::Unnamed(fields) => {
                let mut field_kinds = Vec::new();
                for (i, f) in fields.unnamed.iter().enumerate() {
                    if parse_ignore(&f.attrs)? {
                        continue;
                    }
                    let field_type = &f.ty;
                    let kind_expr = type_to_kind_expr(field_type);
                    let field_tag = parse_tags(&f.attrs)?;
                    let tag_expr = generate_tag_expression(&field_tag);

                    field_kinds.push(quote! {
                        afast::Field { name: format!("_{}", #i), kind: #kind_expr, tag: #tag_expr }
                    });
                }
                Ok(quote! { afast::Kind::Struct { fields: vec![#(#field_kinds),*] } })
            }
            syn::Fields::Unit => Ok(quote! { afast::Kind::Struct { fields: vec![] } }),
        },
        Data::Enum(data_enum) => {
            let mut variant_kinds = Vec::new();
            for variant in &data_enum.variants {
                let name = &variant.ident.to_string();
                let variant_kind = match &variant.fields {
                    syn::Fields::Unit => quote! { (afast::Kind::Unit, #name.to_string()) },
                    syn::Fields::Unnamed(fields) => {
                        let mut field_kinds = Vec::new();
                        for (i, f) in fields.unnamed.iter().enumerate() {
                            if parse_ignore(&f.attrs)? {
                                continue;
                            }
                            let field_type = &f.ty;
                            let kind_expr = type_to_kind_expr(field_type);
                            let field_tag = parse_tags(&f.attrs)?;
                            let tag_expr = generate_tag_expression(&field_tag);

                            field_kinds.push(quote! { afast::Field { name: format!("_{}", #i), kind: #kind_expr, tag: #tag_expr } });
                        }
                        quote! { (afast::Kind::Struct { fields: vec![#(#field_kinds),*] }, #name.to_string()) }
                    }
                    syn::Fields::Named(fields) => {
                        let mut field_kinds = Vec::new();
                        for f in &fields.named {
                            if parse_ignore(&f.attrs)? {
                                continue;
                            }
                            let field_name = f.ident.as_ref().unwrap();
                            let field_type = &f.ty;
                            let kind_expr = type_to_kind_expr(field_type);
                            let field_tag = parse_tags(&f.attrs)?;
                            let tag_expr = generate_tag_expression(&field_tag);

                            field_kinds.push(quote! { afast::Field { name: stringify!(#field_name).to_string(), kind: #kind_expr, tag: #tag_expr } });
                        }
                        quote! { (afast::Kind::Struct { fields: vec![#(#field_kinds),*] }, #name.to_string()) }
                    }
                };
                variant_kinds.push(variant_kind);
            }
            Ok(quote! { afast::Kind::Enum { variants: vec![#(#variant_kinds),*] } })
        }
        Data::Union(_) => Ok(quote! { afast::Kind::Struct { fields: vec![] } }),
    }
}

/// Generates optional `Tag` metadata expression.
fn generate_tag_expression(tag: &Tag) -> proc_macro2::TokenStream {
    let name = tag
        .name
        .as_ref()
        .map(|n| quote! { Some(#n.to_string()) })
        .unwrap_or(quote! { None });
    let description = tag
        .description
        .as_ref()
        .map(|d| quote! { Some(#d.to_string()) })
        .unwrap_or(quote! { None });
    let required = tag
        .required
        .as_ref()
        .map(|r| quote! { Some(#r.to_string()) })
        .unwrap_or(quote! { None });
    let min = tag
        .min
        .as_ref()
        .map(|(v, msg)| quote! { Some((#v, #msg.to_string())) })
        .unwrap_or(quote! { None });
    let max = tag
        .max
        .as_ref()
        .map(|(v, msg)| quote! { Some((#v, #msg.to_string())) })
        .unwrap_or(quote! { None });

    if tag.name.is_none()
        && tag.description.is_none()
        && tag.required.is_none()
        && tag.min.is_none()
        && tag.max.is_none()
    {
        quote! { None }
    } else {
        quote! { Some(afast::Tag { name: #name, description: #description, required: #required, min: #min, max: #max }) }
    }
}

/// Parses field attributes for validation tags.
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
                    Meta::List(meta) => {
                        if meta.path.is_ident("name") {
                            tag.name = Some(meta.parse_args::<LitStr>()?.value());
                        } else if meta.path.is_ident("desc") {
                            tag.description = Some(meta.parse_args::<LitStr>()?.value());
                        } else if meta.path.is_ident("required") {
                            tag.required = Some(meta.parse_args::<LitStr>()?.value());
                        } else if meta.path.is_ident("min") {
                            let inner = meta.parse_args::<IntStrTuple>()?;
                            tag.min = Some((inner.int.base10_parse::<i64>()?, inner.msg.value()));
                        } else if meta.path.is_ident("max") {
                            let inner = meta.parse_args::<IntStrTuple>()?;
                            tag.max = Some((inner.int.base10_parse::<i64>()?, inner.msg.value()));
                        } else {
                            return Err(syn::Error::new(
                                meta.path.span(),
                                format!(
                                    "Unknown validate attribute: {}",
                                    meta.path.get_ident().unwrap()
                                ),
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(tag)
}

/// Converts Rust types to `Kind` expressions for metadata.
fn type_to_kind_expr(ty: &syn::Type) -> proc_macro2::TokenStream {
    match ty {
        syn::Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                let type_name = segment.ident.to_string();
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if type_name == "Option" {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                            let inner_kind = type_to_kind_expr(inner_ty);
                            return quote! { afast::Kind::Nullable(Box::new(#inner_kind)) };
                        }
                    } else if type_name == "Vec" {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                            let inner_kind = type_to_kind_expr(inner_ty);
                            return quote! { afast::Kind::Vec(Box::new(#inner_kind)) };
                        }
                    }
                }
                match type_name.as_str() {
                    "i8" => quote! { afast::Kind::I8 },
                    "i16" => quote! { afast::Kind::I16 },
                    "i32" => quote! { afast::Kind::I32 },
                    "i64" => quote! { afast::Kind::I64 },
                    "i128" => quote! { afast::Kind::I128 },
                    "u8" => quote! { afast::Kind::U8 },
                    "u16" => quote! { afast::Kind::U16 },
                    "u32" => quote! { afast::Kind::U32 },
                    "u64" => quote! { afast::Kind::U64 },
                    "u128" => quote! { afast::Kind::U128 },
                    "f32" => quote! { afast::Kind::F32 },
                    "f64" => quote! { afast::Kind::F64 },
                    "bool" => quote! { afast::Kind::Bool },
                    "String" => quote! { afast::Kind::String },
                    "str" => quote! { afast::Kind::String },
                    "()" => quote! { afast::Kind::Unit },
                    _ => quote! { <#ty as AFastKind>::kind() },
                }
            } else {
                quote! { afast::Kind::String }
            }
        }
        syn::Type::Tuple(type_tuple) => {
            if type_tuple.elems.is_empty() {
                quote! { afast::Kind::Unit }
            } else {
                quote! { afast::Kind::String }
            }
        }
        _ => quote! { afast::Kind::String },
    }
}

fn parse_ignore(attrs: &Vec<Attribute>) -> Result<bool, syn::Error> {
    for attr in attrs {
        if attr.path().is_ident("afast") {
            let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
            for meta in nested {
                match meta {
                    Meta::Path(path) if path.is_ident("ignore") => return Ok(true),
                    _ => {}
                }
            }
        }
    }
    Ok(false)
}
