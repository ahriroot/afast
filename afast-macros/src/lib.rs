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
    Attribute, Data, DeriveInput, Error, LitInt, LitStr, Meta, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
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

    #[cfg(feature = "js")]
    {
        let (serialize_js_code, serialize_js_validate, serialize_js_type, deserialize_js_code) =
            match &input.data {
                Data::Struct(data) => match handler::js::handler_struct(&name, data) {
                    Ok(ts) => ts,
                    Err(e) => return e.to_compile_error().into(),
                },
                Data::Enum(data) => match handler::js::handler_enum(&name, data) {
                    Ok(ts) => ts,
                    Err(e) => return e.to_compile_error().into(),
                },
                Data::Union(_data) => {
                    panic!("unions are not supported yet");
                }
            };

        code.push(quote! {
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
        });
    }

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
