use quote::{format_ident, quote};
use syn::{GenericArgument, PathArguments, Type};

/// Generate serialization code for a given `syn::Type`.
pub fn gen_serialize_code(
    ty: &Type,
    access: proc_macro2::TokenStream,
    depth: usize,
) -> proc_macro2::TokenStream {
    // Match on type kind
    match ty {
        Type::Path(tp) => {
            let seg = tp.path.segments.last().unwrap();
            let ident = seg.ident.to_string();

            // ---- Option<T> ----
            if ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &seg.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        let inner_code = gen_serialize_code(inner_ty, quote!(v), depth + 1);
                        if needs_deref(inner_ty) {
                            return quote! {
                                if let Some(v) = &#access {
                                    buf.push(1);
                                    #inner_code
                                } else {
                                    buf.push(0);
                                }
                            };
                        } else {
                            return quote! {
                                if let Some(v) = #access {
                                    buf.push(1);
                                    #inner_code
                                } else {
                                    buf.push(0);
                                }
                            };
                        }
                    }
                }
            }

            // ---- Vec<T> ----
            if ident == "Vec" {
                if let PathArguments::AngleBracketed(args) = &seg.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        let item_ident = format_ident!("__item_{}", depth);
                        let inner_code =
                            gen_serialize_code(inner_ty, quote!(#item_ident), depth + 1);
                        return quote! {
                            buf.extend(&(#access.len() as u32).to_be_bytes());
                            for #item_ident in #access.iter() {
                                #inner_code
                            }
                        };
                    }
                }
            }

            // ---- HashMap<K,V> ----
            if ident == "HashMap" {
                if let PathArguments::AngleBracketed(args) = &seg.arguments {
                    let mut args_iter = args.args.iter();
                    if let (Some(GenericArgument::Type(k_ty)), Some(GenericArgument::Type(v_ty))) =
                        (args_iter.next(), args_iter.next())
                    {
                        let k_code = gen_serialize_code(k_ty, quote!(k), depth + 1);
                        let v_code = gen_serialize_code(v_ty, quote!(v), depth + 1);
                        return quote! {
                            buf.extend(&(#access.len() as u32).to_be_bytes());
                            for (k, v) in #access.iter() {
                                #k_code
                                #v_code
                            }
                        };
                    }
                }
            }

            // ---- BTreeMap<K,V> ----
            if ident == "BTreeMap" {
                if let PathArguments::AngleBracketed(args) = &seg.arguments {
                    let mut args_iter = args.args.iter();
                    if let (Some(GenericArgument::Type(k_ty)), Some(GenericArgument::Type(v_ty))) =
                        (args_iter.next(), args_iter.next())
                    {
                        let k_code = gen_serialize_code(k_ty, quote!(k), depth + 1);
                        let v_code = gen_serialize_code(v_ty, quote!(v), depth + 1);
                        return quote! {
                            buf.extend(&(#access.len() as u32).to_be_bytes());
                            for (k, v) in #access.iter() {
                                #k_code
                                #v_code
                            }
                        };
                    }
                }
            }

            // ---- Primitive & known simple types ----
            match ident.as_str() {
                "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "u128"
                | "f32" | "f64" => {
                    quote! { buf.extend(&#access.to_be_bytes()); }
                }

                "bool" => {
                    quote! { buf.push(if #access { 1 } else { 0 }); }
                }

                "String" => {
                    quote! {
                        let bytes = #access.as_bytes();
                        buf.extend(&(bytes.len() as u32).to_be_bytes());
                        buf.extend(bytes);
                    }
                }

                _ => {
                    // ---- Custom type ----
                    quote! {
                        buf.extend(#access.to_bytes());
                    }
                }
            }
        }

        // ---- Tuple, array, etc. ----
        Type::Array(arr) => {
            let elem_ty = &*arr.elem;
            let idx = format_ident!("__i_{}", depth);
            let inner_code = gen_serialize_code(elem_ty, quote!(#idx), depth + 1);
            quote! {
                for #idx in #access.iter() {
                    #inner_code
                }
            }
        }

        _ => {
            quote! {
                compile_error!("Unsupported type for serialization");
            }
        }
    }
}

fn needs_deref(ty: &Type) -> bool {
    if let Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            let ident = seg.ident.to_string();
            match ident.as_str() {
                "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "u128"
                | "f32" | "f64" | "bool" => {
                    // 基本类型，实现了 Copy，不需要解引用
                    return false;
                }
                _ => {
                    // 其他类型（String、自定义类型等）需要解引用
                    return true;
                }
            }
        }
    }
    true // 默认情况下需要解引用
}
