use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    FnArg, ItemFn, LitStr, Meta, Pat, PatType, ReturnType, Type, parse_macro_input,
    punctuated::Punctuated, token,
};

/// Attribute macro to convert an async function into a generic binary handler.
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let ident = &input.sig.ident;
    let vis = &input.vis;
    let block = &input.block;
    let sig = &input.sig;

    // 提取原始函数的参数名和类型
    let mut params = Vec::new();
    let mut state_ty = None;
    let mut header_ty = None;
    let mut req_ty = None;

    for (i, arg) in sig.inputs.iter().enumerate() {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
            let param_name = match &**pat {
                Pat::Ident(ident) => &ident.ident,
                _ => panic!("Expected identifier pattern"),
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

    let state_ty = state_ty.expect("expected state parameter");
    let header_ty = header_ty.expect("expected header parameter");
    let req_ty = req_ty.expect("expected request parameter");

    // 使用原始参数名重构 inner_ident 函数
    let inner_ident = syn::Ident::new(&format!("__inner_{}", ident), ident.span());

    // 构建参数列表，保持原始参数名
    let inner_params: Vec<_> = params
        .iter()
        .map(|(name, ty)| {
            quote! { #name: #ty }
        })
        .collect();

    let ret_type = match &sig.output {
        syn::ReturnType::Type(_, ty) => (**ty).clone(),
        syn::ReturnType::Default => syn::parse_quote!(()),
    };

    // 解析返回类型 Result<Resp, Error> 中的 Resp
    let resp_ty = match &sig.output {
        ReturnType::Type(_, ty) => {
            if let Type::Path(path) = &**ty {
                if let Some(seg) = path.path.segments.first() {
                    if seg.ident == "Result" {
                        if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                            if let Some(syn::GenericArgument::Type(resp_ty)) = args.args.first() {
                                resp_ty.clone()
                            } else {
                                panic!("Result must have Ok type");
                            }
                        } else {
                            panic!("Result must have angle brackets");
                        }
                    } else {
                        panic!("Return type must be Result<Resp, Error>");
                    }
                } else {
                    panic!("Return type path empty");
                }
            } else {
                panic!("Return type must be a type path");
            }
        }
        ReturnType::Default => panic!("Handler must return Result<Resp, Error>"),
    };

    let func_name = ident.to_string();
    let args = parse_macro_input!(attr with Punctuated<Meta, token::Comma>::parse_terminated);
    let mut desc = String::new();
    let mut mw = quote! {
        Box::new(|state: #state_ty, header: #header_ty| {
            Box::pin(async move {
                Ok(())
            })
        })
    };
    let mut namespace: Vec<String> = Vec::new();

    for arg in args {
        match arg {
            Meta::Path(_) => {}
            Meta::List(meta) => {
                if meta.path.is_ident("desc") {
                    if let Ok(lit) = meta.parse_args::<LitStr>() {
                        desc = format!(" * {}\n", lit.value());
                    }
                }
                if meta.path.is_ident("mw") {
                    if let Ok(lit) = meta.parse_args::<LitStr>() {
                        let ident = format_ident!("{}", lit.value().trim());
                        mw = quote! {
                            Box::new(|state: #state_ty, header: #header_ty| {
                                Box::pin(async move {
                                    match #ident(state, header).await {
                                        Ok(_) => Ok(()),
                                        Err(e) => Err(afast::Error::server_error(500, e.to_string())),
                                    }
                                })
                            })
                        };
                    }
                }
                if meta.path.is_ident("ns") {
                    if let Ok(lit) = meta.parse_args::<LitStr>() {
                        namespace = lit
                            .value()
                            .split(".")
                            .map(|s| s.trim().to_string())
                            .collect();
                    }
                }
            }
            Meta::NameValue(_) => {}
        }
    }

    #[cfg(feature = "js")]
    let js = quote! {
        let code_request_type = #req_ty::to_js_type("request").to_string();
        let code_resposne_type = #resp_ty::to_js_type("response").to_string();
        js.push(format!("/**\n{} * @param {{{}}} request\n * @returns {{{}}}\n */\n", #desc, code_request_type, code_resposne_type));
        js.push(format!("{}: async (request) => {{", #func_name));
        let code = #req_ty::to_js_validate("request").to_string();
        js.push(code);
        js.push("const _b1 = new AFastByteBuffer();".to_string());
        js.push("const _header = await this._header();".to_string());
        let code = #header_ty::to_js("_header").to_string();
        js.push(code);
        js.push(format!("_b1.pU32({}+this.offset);", id));
        let code = #req_ty::to_js("request").to_string();
        js.push(code);
        js.push("const _b2 = new AFastByteReader(await this._call(_b1.tU8A()));".to_string());
        js.push("_b2.rI32();".to_string());
        let code = format!("const response = {};", #resp_ty::from_js("response").to_string());
        js.push(code);
        js.push("return response;".to_string());
        js.push("}".to_string());

        let code_request_type = #req_ty::to_js_type("request").to_string();
        let code_resposne_type = #resp_ty::to_js_type("response").to_string();
        ts.push(format!("/**\n{} * @param {{{}}} request\n * @returns {{{}}}\n */\n", #desc, code_request_type, code_resposne_type));
        ts.push(format!("{}: async (request:{}): Promise<{}> => {{", #func_name, code_request_type, code_resposne_type));
        let code = #req_ty::to_js_validate("request").to_string();
        ts.push(code);
        ts.push("const _b1 = new AFastByteBuffer();".to_string());
        ts.push("const _header = await this._header();".to_string());
        let code = #header_ty::to_js("_header").to_string();
        ts.push(code);
        ts.push(format!("_b1.pU32({}+this.offset);", id));
        let code = #req_ty::to_js("request").to_string();
        ts.push(code);
        ts.push("const _b2 = new AFastByteReader(await this._call(_b1.tU8A()));".to_string());
        ts.push("_b2.rI32();".to_string());
        let code = format!("const response = {};", #resp_ty::from_js("response").to_string());
        ts.push(code);
        ts.push("return response as any;".to_string());
        ts.push("}".to_string());
    };

    #[cfg(not(feature = "js"))]
    let js = quote! {};

    let expanded = quote! {
        /// The inner async function preserving the original user logic and parameter names.
        #vis async fn #inner_ident(#(#inner_params),*) -> #ret_type {
            #block
        }

        /// Returns a boxed handler suitable for AFast registration.
        #vis fn #ident(id: u32) -> (
            String,
            String,
            Box<
                dyn Fn(
                    #state_ty,
                    #header_ty,
                ) -> std::pin::Pin<
                    Box<dyn std::future::Future<Output = Result<(), afast::Error>> + Send>
                > + Send + Sync + 'static,
            >,
            Vec<String>,
            Box<
                dyn Fn(
                    #state_ty,
                    #header_ty,
                    &[u8],
                ) -> std::pin::Pin<
                    Box<dyn std::future::Future<Output = Result<Vec<u8>, afast::Error>> + Send>
                > + Send + Sync + 'static,
            >,
        ) {
            let mut js: Vec<String> = Vec::new();
            let mut ts: Vec<String> = Vec::new();
            #js

            (
                js.join(""),
                ts.join(""),
                #mw,
                vec![#(#namespace.to_string()),*],
                Box::new(|state: #state_ty, header: #header_ty, req: &[u8]| {
                    let req = #req_ty::from_bytes(req);
                    Box::pin(async move {
                        match req {
                            Ok((req, _)) => {
                                req.validate().map_err(|e| afast::Error::client_error(400, e.join(",")))?;
                                // 调用 inner 函数时使用原始参数名
                                match #inner_ident(state, header, req).await {
                                    Ok(resp) => Ok(resp.to_bytes()),
                                    Err(e) => Err(afast::Error::server_error(500, e.to_string())),
                                }
                            },
                            Err(e) => return Err(afast::Error::client_error(400, e.to_string())),
                        }
                    })
                }),
            )
        }
    };

    TokenStream::from(expanded)
}

/// Macro to register multiple handlers.
pub fn register(input: TokenStream) -> TokenStream {
    let funcs: syn::punctuated::Punctuated<syn::Ident, syn::token::Comma> =
        parse_macro_input!(input with syn::punctuated::Punctuated::parse_terminated);

    let mut registrations = vec![];
    for (id, func) in funcs.iter().enumerate() {
        let id = id as u32;
        let name = func.to_string();

        registrations.push(quote! {
            {
                let (js, ts, middleware, namespace, func) = #func(#id);
                afast::HandlerGeneric {
                    id: #id,
                    name: #name,
                    js,
                    ts,
                    middleware,
                    namespace,
                    func,
                }
            }
        });
    }

    let expanded = quote! {
        vec![#(#registrations), *]
    };

    TokenStream::from(expanded)
}
