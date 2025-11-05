use quote::{format_ident, quote};
use syn::Type;

/// Recursively generates serialization code for a given type and access path.
///
/// # Arguments
/// * `ty_str` - The type as a string (e.g., "i32", "Vec<String>", "Option<i64>")
/// * `access` - The expression to access the value (e.g., `self.field`)
/// * `depth` - Current recursion depth to generate unique variable names
///
/// # Returns
/// A `TokenStream` that serializes the given value into `buf`.
pub fn gen_js_code(
    ty_str: &str,
    access: &proc_macro2::TokenStream,
    prefix: &proc_macro2::TokenStream,
    depth: usize,
    ty: &Type,
) -> proc_macro2::TokenStream {
    let ty_str = ty_str.replace(' ', "");

    // Option<T> handling
    if ty_str.starts_with("Option<") {
        let inner = &ty_str[7..ty_str.len() - 1];

        let code = format!("if({{}}.{} === null)_b1.pU8(0);", access);

        let else_code = gen_js_code(inner, access, prefix, depth + 1, ty);

        return quote! {
            code.push_str(&format!(#code, prefix));
            code.push_str("else{_b1.pU8(1);");
            #else_code
            code.push_str("}");
        };
    }

    // Vec<T> handling (recursive)
    if ty_str.starts_with("Vec<") {
        let inner = &ty_str[4..ty_str.len() - 1]; // 内部类型
        let inner_ty: Type = syn::parse_str(inner).unwrap(); // 构造 Type

        let item_ident = format_ident!("__item_{}", depth);
        let inner_code = gen_js_code(
            inner,
            &quote!(#item_ident),
            &proc_macro2::TokenStream::new(),
            depth + 1,
            &inner_ty,
        );

        let code = format!(
            "_b1.pU32({}.{}.length);for(let {} of {}.{}){{",
            prefix, access, item_ident, prefix, access
        );

        return quote! {
            code.push_str(#code);
            #inner_code
            code.push_str("}");
        };
    }

    // // Map types handling HashMap<K,V>
    // if ty_str.starts_with("HashMap<") {
    //     let inner = &ty_str[8..ty_str.len() - 1];
    //     let mut parts = inner.splitn(2, ',');
    //     let key_ty = parts.next().unwrap_or("").trim();
    //     let val_ty = parts.next().unwrap_or("").trim();
    //     let key_access = if key_ty == "bool" {
    //         quote!(*k)
    //     } else {
    //         quote!(k)
    //     };
    //     let val_access = if val_ty == "bool" {
    //         quote!(*v)
    //     } else {
    //         quote!(v)
    //     };
    //     let key_code = gen_serialize_code(key_ty, key_access, depth + 1);
    //     let val_code = gen_serialize_code(val_ty, val_access, depth + 1);
    //     return quote! {
    //         _b1.extend(&(#access.len() as u32).to_be_bytes()); // Serialize map length
    //         for (k, v) in #access.iter() {
    //             #key_code
    //             #val_code
    //         }
    //     };
    // }

    // // Map types handling BTreeMap<K,V>
    // if ty_str.starts_with("BTreeMap<") {
    //     let inner = &ty_str[9..ty_str.len() - 1];
    //     let mut parts = inner.splitn(2, ',');
    //     let key_ty = parts.next().unwrap_or("").trim();
    //     let val_ty = parts.next().unwrap_or("").trim();
    //     let key_access = if key_ty == "bool" {
    //         quote!(*k)
    //     } else {
    //         quote!(k)
    //     };
    //     let val_access = if val_ty == "bool" {
    //         quote!(*v)
    //     } else {
    //         quote!(v)
    //     };
    //     let key_code = gen_serialize_code(key_ty, key_access, depth + 1);
    //     let val_code = gen_serialize_code(val_ty, val_access, depth + 1);
    //     return quote! {
    //         _b1.extend(&(#access.len() as u32).to_be_bytes()); // Serialize map length
    //         for (k, v) in #access.iter() {
    //             #key_code
    //             #val_code
    //         }
    //     };
    // }

    // Primitive types, boolean, string, and nested structs
    match ty_str.as_str() {
        "i8" => {
            if prefix.is_empty() {
                let code = format!("_b1.pI8({});", access);
                quote! {
                    code.push_str(#code);
                }
            } else {
                let code = format!("_b1.pI8({{}}.{});", access);
                quote! {
                    code.push_str(&format!(#code, prefix));
                }
            }
        }
        "i16" => {
            if prefix.is_empty() {
                let code = format!("_b1.pI16({});", access);
                quote! {
                    code.push_str(#code);
                }
            } else {
                let code = format!("_b1.pI16({{}}.{});", access);
                quote! {
                    code.push_str(&format!(#code, prefix));
                }
            }
        }
        "i32" => {
            if prefix.is_empty() {
                let code = format!("_b1.pI32({});", access);
                quote! {
                    code.push_str(#code);
                }
            } else {
                let code = format!("_b1.pI32({{}}.{});", access);
                quote! {
                    code.push_str(&format!(#code, prefix));
                }
            }
        }
        "i64" => {
            if prefix.is_empty() {
                let code = format!("_b1.pI64({});", access);
                quote! {
                    code.push_str(#code);
                }
            } else {
                let code = format!("_b1.pI64({{}}.{});", access);
                quote! {
                    code.push_str(&format!(#code, prefix));
                }
            }
        }
        "i128" => {
            if prefix.is_empty() {
                let code = format!("_b1.pI128({});", access);
                quote! {
                    code.push_str(#code);
                }
            } else {
                let code = format!("_b1.pI128({{}}.{});", access);
                quote! {
                    code.push_str(&format!(#code, prefix));
                }
            }
        }
        "isize" => {
            if prefix.is_empty() {
                let code = format!("_b1.putIsize({});", access);
                quote! {
                    code.push_str(#code);
                }
            } else {
                let code = format!("_b1.putIsize({{}}.{});", access);
                quote! {
                    code.push_str(&format!(#code, prefix));
                }
            }
        }
        "u8" => {
            if prefix.is_empty() {
                let code = format!("_b1.pU8({});", access);
                quote! {
                    code.push_str(#code);
                }
            } else {
                let code = format!("_b1.pU8({{}}.{});", access);
                quote! {
                    code.push_str(&format!(#code, prefix));
                }
            }
        }
        "u16" => {
            if prefix.is_empty() {
                let code = format!("_b1.pU16({});", access);
                quote! {
                    code.push_str(#code);
                }
            } else {
                let code = format!("_b1.pU16({{}}.{});", access);
                quote! {
                    code.push_str(&format!(#code, prefix));
                }
            }
        }
        "u32" => {
            if prefix.is_empty() {
                let code = format!("_b1.pU32({});", access);
                quote! {
                    code.push_str(#code);
                }
            } else {
                let code = format!("_b1.pU32({{}}.{});", access);
                quote! {
                    code.push_str(&format!(#code, prefix));
                }
            }
        }
        "u64" => {
            if prefix.is_empty() {
                let code = format!("_b1.pU64({});", access);
                quote! {
                    code.push_str(#code);
                }
            } else {
                let code = format!("_b1.pU64({{}}.{});", access);
                quote! {
                    code.push_str(&format!(#code, prefix));
                }
            }
        }
        "u128" => {
            if prefix.is_empty() {
                let code = format!("_b1.pU128({});", access);
                quote! {
                    code.push_str(#code);
                }
            } else {
                let code = format!("_b1.pU128({{}}.{});", access);
                quote! {
                    code.push_str(&format!(#code, prefix));
                }
            }
        }
        "usize" => {
            if prefix.is_empty() {
                let code = format!("_b1.putUsize({});", access);
                quote! {
                    code.push_str(#code);
                }
            } else {
                let code = format!("_b1.putUsize({{}}.{});", access);
                quote! {
                    code.push_str(&format!(#code, prefix));
                }
            }
        }
        "f32" => {
            if prefix.is_empty() {
                let code = format!("_b1.pF32({});", access);
                quote! {
                    code.push_str(#code);
                }
            } else {
                let code = format!("_b1.pF32({{}}.{});", access);
                quote! {
                    code.push_str(&format!(#code, prefix));
                }
            }
        }
        "f64" => {
            if prefix.is_empty() {
                let code = format!("_b1.pF64({});", access);
                quote! {
                    code.push_str(#code);
                }
            } else {
                let code = format!("_b1.pF64({{}}.{});", access);
                quote! {
                    code.push_str(&format!(#code, prefix));
                }
            }
        }
        "bool" => {
            if prefix.is_empty() {
                let code = format!("_b1.pB({});", access);
                quote! {
                    code.push_str(#code);
                }
            } else {
                let code = format!("_b1.pB({{}}.{});", access);
                quote! {
                    code.push_str(&format!(#code, prefix));
                }
            }
        }
        "String" => {
            if prefix.is_empty() {
                let code = format!("_b1.pS({});", access);
                quote! {
                    code.push_str(#code);
                }
            } else {
                let code = format!("_b1.pS({{}}.{});", access);
                quote! {
                    code.push_str(&format!(#code, prefix));
                }
            }
        }
        _ => {
            // Fallback for nested custom types implementing `to_bytes()`
            if prefix.is_empty() {
                let access = access.to_string();
                let prefix = format_ident!("__prefix_{}", depth);
                quote! {
                    let #prefix = format!("{}", #access);
                    code.push_str(&#ty::to_js(&#prefix));
                }
            } else {
                let access = access.to_string();
                let prefix = format_ident!("__prefix_{}", depth);
                quote! {
                    let #prefix = format!("{}.{}", prefix, #access);
                    code.push_str(&#ty::to_js(&#prefix));
                }
            }
        }
    }
}
