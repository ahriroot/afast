use proc_macro::TokenStream;
use quote::quote;
use syn::{
    FnArg, ItemFn, LitStr, Meta, Pat, PatType, parse_macro_input, punctuated::Punctuated, token,
};

/// Attribute macro that converts an asynchronous Rust function into a fully-featured
/// AFast handler with automatic support for:
/// - Binary serialization/deserialization
/// - Input validation
/// - Middleware execution
/// - JavaScript/TypeScript/Kotlin/Java client generation, etc.
/// - API documentation
///
/// # Usage
///
/// ```rust
/// #[handler(desc("Get user information"), ns("api.user"))]
/// async fn get_user(
///     state: Arc<Mutex<String>>,
///     header: Header,
///     req: Request,
/// ) -> Result<Response, Error> {
///     Ok(Response { ... })
/// }
/// ```
///
/// The macro generates:
/// 1. An internal async function preserving the original logic (`__inner_*`)
/// 2. A wrapper function returning:
///    - Middleware executor
///    - Namespaced handler identifiers
///    - Binary request/response handlers
///
/// # Macro Attributes
///
/// - `desc("...")` — API description for documentation and client generation
/// - `ns("...")` — Namespace path for nested client generation (dot-separated)
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let ident = &input.sig.ident;
    let vis = &input.vis;
    let block = &input.block;
    let sig = &input.sig;

    // Extract parameter types: state, header, request
    let mut params = Vec::new();
    let mut state_ty = None;
    let mut header_ty = None;
    let mut req_ty = None;

    for (i, arg) in sig.inputs.iter().enumerate() {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
            let param_name = match &**pat {
                Pat::Ident(ident) => &ident.ident,
                _ => panic!("Expected identifier pattern for function parameters"),
            };

            params.push((param_name, ty.clone()));

            match i {
                0 => state_ty = Some(ty.clone()),
                1 => header_ty = Some(ty.clone()),
                2 => req_ty = Some(ty.clone()),
                _ => {}
            }
        }
    }

    let state_ty = state_ty.expect("Expected first parameter to be state");
    let header_ty = header_ty.expect("Expected second parameter to be header");
    let req_ty = req_ty.expect("Expected third parameter to be request");

    // Generate internal function name to preserve original logic
    let inner_ident = syn::Ident::new(&format!("__inner_{}", ident), ident.span());

    // Build parameter token streams for the internal function
    let inner_params: Vec<_> = params
        .iter()
        .map(|(name, ty)| quote! { #name: #ty })
        .collect();

    // Determine return type of the original function
    let ret_type = match &sig.output {
        syn::ReturnType::Type(_, ty) => (**ty).clone(),
        syn::ReturnType::Default => syn::parse_quote!(()),
    };

    #[cfg(feature = "code")]
    let resp_ty = match &sig.output {
        syn::ReturnType::Type(_, ty) => {
            if let syn::Type::Path(path) = &**ty {
                if let Some(seg) = path.path.segments.first() {
                    if seg.ident == "Result" {
                        if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                            if let Some(syn::GenericArgument::Type(resp_ty)) = args.args.first() {
                                resp_ty.clone()
                            } else {
                                panic!("Result must specify Ok type")
                            }
                        } else {
                            panic!("Result must use angle brackets")
                        }
                    } else {
                        panic!("Handler return type must be Result<Resp, Error>")
                    }
                } else {
                    panic!("Return type path is empty")
                }
            } else {
                panic!("Handler return type must be a type path")
            }
        }
        syn::ReturnType::Default => panic!("Handler must return Result<Resp, Error>"),
    };

    #[cfg(feature = "code")]
    let func_name = ident.to_string();
    let args = parse_macro_input!(attr with Punctuated<Meta, token::Comma>::parse_terminated);
    #[cfg(feature = "code")]
    let mut desc = String::new();
    let mut namespace: Vec<String> = Vec::new();

    for arg in args {
        match arg {
            Meta::Path(_) => {}
            Meta::List(meta) => {
                #[cfg(feature = "code")]
                if meta.path.is_ident("desc") {
                    if let Ok(lit) = meta.parse_args::<LitStr>() {
                        desc = lit.value();
                    }
                }
                if meta.path.is_ident("ns") {
                    if let Ok(lit) = meta.parse_args::<LitStr>() {
                        namespace = lit
                            .value()
                            .split('.')
                            .map(|s| s.trim().to_string())
                            .collect();
                    }
                }
            }
            Meta::NameValue(_) => {}
        }
    }

    let mut wrapper_args: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut wrapper_returns: Vec<proc_macro2::TokenStream> = Vec::new();

    #[cfg(feature = "js")]
    {
        let has_namespace = !namespace.is_empty();
        wrapper_args.push(quote! { String });
        wrapper_returns.push(quote! {
            {
                let desc = if #desc.is_empty() {"".to_string()} else {format!(" * {}\n", #desc)};
                let mut js: Vec<String> = Vec::new();
                let kind_request = #req_ty::kind();
                let kind_response = #resp_ty::kind();
                let code_request_type = kind_request.gen_js_type();
                let code_resposne_type = kind_response.gen_js_type();
                js.push(format!("/**\n{} * @param {{{}}} request\n * @returns {{{}}}\n */\n", desc, code_request_type, code_resposne_type));
                js.push(format!("{}{}async(request)=>{{", #func_name, if #has_namespace{":"}else{"="}));
                let code = kind_request.gen_js_validate("request", None, 0).to_string();
                js.push(code);
                js.push("const _b1=new AFastByteBuffer();".to_string());
                js.push("const _header=await this._header();".to_string());
                let code = #header_ty::kind().gen_js_to_bytes("_header", 0);
                js.push(code);
                js.push(format!("_b1.pU32({}+this.offset);", id));
                let code = kind_request.gen_js_to_bytes("request", 0);
                js.push(code);
                js.push("const _b2=new AFastByteReader(await this._call(_b1.tU8A()));".to_string());
                js.push("_b2.rI32();".to_string());
                let code = #header_ty::field("_header2").gen_bytes_to_js(0);
                js.push(code);
                js.push("this._hook?.(_header2);".to_string());
                let code = #resp_ty::field("response").gen_bytes_to_js(0);
                js.push(code);
                js.push("return response;}".to_string());
                js.join("")
            }
        });
    }

    #[cfg(feature = "ts")]
    {
        let has_namespace = !namespace.is_empty();
        wrapper_args.push(quote! { String });
        wrapper_returns.push(quote! {
            {
                let desc = if #desc.is_empty() {"".to_string()} else {format!(" * {}\n", #desc)};
                let mut ts: Vec<String> = Vec::new();
                let kind_request = #req_ty::kind();
                let kind_response = #resp_ty::kind();
                let code_request_type = kind_request.gen_ts_type();
                let code_resposne_type = kind_response.gen_ts_type();
                ts.push(format!("/**\n{} * @param {{{}}} request\n * @returns {{{}}}\n */\n", desc, code_request_type, code_resposne_type));
                ts.push(format!("{}{}async(request:{}):Promise<{}>=>{{", #func_name, if #has_namespace{":"}else{"="}, code_request_type, code_resposne_type));
                let code = kind_request.gen_ts_validate("request", None, 0).to_string();
                ts.push(code);
                ts.push("const _b1=new AFastByteBuffer();".to_string());
                ts.push("const _header=await this._header();".to_string());
                let code = #header_ty::kind().gen_ts_to_bytes("_header", 0);
                ts.push(code);
                ts.push(format!("_b1.pU32({}+this.offset);", id));
                let code = kind_request.gen_ts_to_bytes("request", 0).to_string();
                ts.push(code);
                ts.push("const _b2=new AFastByteReader(await this._call(_b1.tU8A()));".to_string());
                ts.push("_b2.rI32();".to_string());
                let code = #header_ty::field("_header2").gen_bytes_to_ts(0);
                ts.push(code);
                ts.push("this._hook?.(_header2);".to_string());
                let code = #resp_ty::field("response").gen_bytes_to_ts(0);
                ts.push(code);
                ts.push("return response as any;}".to_string());
                ts.join("")
            }
        });
    }

    #[cfg(feature = "doc")]
    {
        let ns = namespace
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(",");
        wrapper_args.push(quote! { String });
        wrapper_returns.push(quote! {
            {
                let mut doc: Vec<String> = Vec::new();
                let kind_request = #req_ty::field("request");
                let kind_response = #resp_ty::field("response");
                let type_request = kind_request.gen_js_type();
                let type_response = kind_response.gen_js_type();
                let doc_request = kind_request.gen_doc();
                let doc_response = kind_response.gen_doc();
                format!(
                    r#"{{"name":"{}","desc":"{}","ns":[{}],"request":{},"response":{},"req_type":"{}","resp_type":"{}"}}"#,
                    #func_name,
                    #desc,
                    #ns,
                    doc_request,
                    doc_response,
                    type_request,
                    type_response
                )
            }
        });
    }

    wrapper_args.push(quote! { Vec<String> });
    wrapper_args.push(quote! {
        afast::Processor<#state_ty, #header_ty>
    });
    wrapper_returns.push(quote! {vec![#(#namespace.to_string()),*]});
    wrapper_returns.push(quote! {
        afast::Processor::Handler(Box::new(|state: #state_ty, header: #header_ty, req: &[u8]| {
            let req = #req_ty::from_bytes(req);
            Box::pin(async move {
                match req {
                    Ok((req, _)) => {
                        req.validate().map_err(|e| afast::Error::custom_error(400, e.join(",")))?;
                        match #inner_ident(state, header, req).await {
                            Ok(resp) => Ok(resp.to_bytes()),
                            Err(e) => Err(e),
                        }
                    },
                    Err(e) => return Err(e),
                }
            })
        }))
    });

    let expanded = quote! {
        /// Internal async function preserving the original user logic.
        #vis async fn #inner_ident(#(#inner_params),*) -> #ret_type {
            #block
        }

        /// Generates a wrapper suitable for AFast handler registration.
        #vis fn #ident(id: u32) -> (#( #wrapper_args ),*) {
            (#( #wrapper_returns ),*)
        }
    };

    TokenStream::from(expanded)
}

/// Macro to register multiple handlers at once.
///
/// # Usage
///
/// ```rust
/// let handlers = register!{get_user, get_id};
/// ```
///
/// Each function in the list must be annotated with `#[handler]`.
/// The macro returns a `Vec<afast::HandlerGeneric>` with automatically
/// assigned IDs and all wrapper components needed for AFast server registration.
pub fn register(input: TokenStream) -> TokenStream {
    let funcs: syn::punctuated::Punctuated<syn::Ident, syn::token::Comma> =
        parse_macro_input!(input with syn::punctuated::Punctuated::parse_terminated);

    let mut registrations = vec![];
    for (id, func) in funcs.iter().enumerate() {
        let id = id as u32;
        let name = func.to_string();

        let mut wrapper_args: Vec<proc_macro2::TokenStream> = Vec::new();

        #[cfg(feature = "js")]
        wrapper_args.push(quote! { js });

        #[cfg(feature = "ts")]
        wrapper_args.push(quote! { ts });

        #[cfg(feature = "doc")]
        wrapper_args.push(quote! { doc });

        wrapper_args.push(quote! {namespace, func});

        registrations.push(quote! {
            {
                let (#( #wrapper_args ),*) = #func(#id);
                afast::HandlerGeneric {
                    id: #id,
                    name: #name,
                    #( #wrapper_args ),*
                }
            }
        });
    }

    let expanded = quote! {
        vec![#(#registrations), *]
    };

    TokenStream::from(expanded)
}
