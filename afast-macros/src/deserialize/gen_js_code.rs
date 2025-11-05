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
        let else_code = gen_js_code(inner, access, prefix, depth + 1, ty);

        let code = format!("_b2.rU8() === 0 ? null : ");

        return quote! {
            code.push_str(#code);
            #else_code
        };
    }

    // Vec<T> handling (recursive)
    if ty_str.starts_with("Vec<") {
        let item_ident = format_ident!("__item_{}", depth);
        let inner = &ty_str[4..ty_str.len() - 1];
        let inner_ty: Type = syn::parse_str(inner).unwrap();
        let inner_code = gen_js_code(
            inner,
            &quote!(#item_ident),
            &proc_macro2::TokenStream::new(),
            depth + 1,
            &inner_ty,
        );

        return quote! {
            code.push_str("Array.from({length:_b2.rU32()},()=>(");
            #inner_code
            code.push_str("))");
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
    //         buf.extend(&(#access.len() as u32).to_be_bytes()); // Serialize map length
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
    //         buf.extend(&(#access.len() as u32).to_be_bytes()); // Serialize map length
    //         for (k, v) in #access.iter() {
    //             #key_code
    //             #val_code
    //         }
    //     };
    // }

    // Primitive types, boolean, string, and nested structs
    match ty_str.as_str() {
        "i8" => {
            let code = format!("_b2.rI8()");
            quote! {
                code.push_str(#code);
            }
        }
        "i16" => {
            let code = format!("_b2.rI16()");
            quote! {
                code.push_str(#code);
            }
        }
        "i32" => {
            let code = format!("_b2.rI32()");
            quote! {
                code.push_str(#code);
            }
        }
        "i64" => {
            let code = format!("_b2.rI64()");
            quote! {
                code.push_str(#code);
            }
        }
        "i128" => {
            let code = format!("_b2.rI128();");
            quote! {
                code.push_str(#code);
            }
        }
        "isize" => {
            let code = format!("_b2.readIsize()");
            quote! {
                code.push_str(#code);
            }
        }
        "u8" => {
            let code = format!("_b2.rU8()");
            quote! {
                code.push_str(#code);
            }
        }
        "u16" => {
            let code = format!("_b2.rU16()");
            quote! {
                code.push_str(#code);
            }
        }
        "u32" => {
            let code = format!("_b2.rU32()");
            quote! {
                code.push_str(#code);
            }
        }
        "u64" => {
            let code = format!("_b2.rU64()");
            quote! {
                code.push_str(#code);
            }
        }
        "u128" => {
            let code = format!("_b2.rU128()");
            quote! {
                code.push_str(#code);
            }
        }
        "usize" => {
            let code = format!("_b2.readUsize()");
            quote! {
                code.push_str(#code);
            }
        }
        "f32" => {
            let code = format!("_b2.rF32()");
            quote! {
                code.push_str(#code);
            }
        }
        "f64" => {
            let code = format!("_b2.rF64()");
            quote! {
                code.push_str(#code);
            }
        }
        "bool" => {
            let code = format!("_b2.rB()");
            quote! {
                code.push_str(#code);
            }
        }
        "String" => {
            let code = format!("_b2.rS()");
            quote! {
                code.push_str(#code);
            }
        }
        _ => {
            let access = access.to_string();
            quote! {
                let prefix = format!("{}", #access);
                code.push_str(&format!("{}", #ty::from_js(&prefix)));
            }
        }
    }
}
