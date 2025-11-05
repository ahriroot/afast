use quote::{format_ident, quote};
use syn::{Type, PathArguments};

pub fn gen_deserialize_code(
    ty: &Type,
    target: proc_macro2::TokenStream,
    depth: usize,
) -> proc_macro2::TokenStream {
    // 只处理 Type::Path 及常见情况，其他情况抛错以便发现
    match ty {
        // Option<T>
        Type::Path(tp) if !tp.path.segments.is_empty() && tp.path.segments.last().unwrap().ident == "Option" => {
            let seg = tp.path.segments.last().unwrap();
            if let PathArguments::AngleBracketed(args) = &seg.arguments {
                let inner_ty = match args.args.first().expect("Option must have 1 arg") {
                    syn::GenericArgument::Type(t) => t,
                    _ => panic!("Unsupported Option inner argument"),
                };
                let inner_code = gen_deserialize_code(inner_ty, quote!(val), depth + 1);
                quote! {
                    let #target = if buf[_offset] == 1 {
                        _offset += 1;
                        #inner_code
                        Some(val)
                    } else {
                        _offset += 1;
                        None
                    };
                }
            } else {
                panic!("Invalid Option<> arguments");
            }
        }

        // Vec<T>
        Type::Path(tp) if !tp.path.segments.is_empty() && tp.path.segments.last().unwrap().ident == "Vec" => {
            let seg = tp.path.segments.last().unwrap();
            if let PathArguments::AngleBracketed(args) = &seg.arguments {
                let inner_ty = match args.args.first().expect("Vec must have 1 arg") {
                    syn::GenericArgument::Type(t) => t,
                    _ => panic!("Unsupported Vec inner argument"),
                };

                // 先创建 item ident，再生成 inner_code（避免用到未定义的 ident）
                let target_item = format_ident!("__item_{}", depth);
                let inner_code = gen_deserialize_code(inner_ty, quote!(#target_item), depth + 1);

                quote! {
                    let mut #target = Vec::new();
                    if _offset + 4 > _length { return Err(::afast::Error::DecodeError); }
                    let mut len_bytes = [0u8; 4];
                    len_bytes.copy_from_slice(&buf[_offset.._offset+4]);
                    _offset += 4;
                    let len = u32::from_be_bytes(len_bytes) as usize;

                    for _ in 0..len {
                        #inner_code
                        #target.push(#target_item);
                    }
                }
            } else {
                panic!("Invalid Vec<> arguments");
            }
        }

        // HashMap<K,V>
        Type::Path(tp) if !tp.path.segments.is_empty() && tp.path.segments.last().unwrap().ident == "HashMap" => {
            let seg = tp.path.segments.last().unwrap();
            if let PathArguments::AngleBracketed(args) = &seg.arguments {
                let mut iter = args.args.iter();
                let key_ty = match iter.next().expect("HashMap needs key").clone() {
                    syn::GenericArgument::Type(t) => t,
                    _ => panic!("Unsupported HashMap key arg"),
                };
                let val_ty = match iter.next().expect("HashMap needs val").clone() {
                    syn::GenericArgument::Type(t) => t,
                    _ => panic!("Unsupported HashMap val arg"),
                };

                let k_ident = format_ident!("__k_{}", depth);
                let v_ident = format_ident!("__v_{}", depth);
                let key_code = gen_deserialize_code(&key_ty, quote!(#k_ident), depth + 1);
                let val_code = gen_deserialize_code(&val_ty, quote!(#v_ident), depth + 1);

                quote! {
                    let mut #target = ::std::collections::HashMap::new();
                    if _offset + 4 > _length { return Err(::afast::Error::DecodeError); }
                    let mut len_bytes = [0u8; 4];
                    len_bytes.copy_from_slice(&buf[_offset.._offset+4]);
                    _offset += 4;
                    let len = u32::from_be_bytes(len_bytes) as usize;

                    for _ in 0..len {
                        #key_code
                        #val_code
                        #target.insert(#k_ident, #v_ident);
                    }
                }
            } else {
                panic!("Invalid HashMap<> args");
            }
        }

        // BTreeMap<K,V>
        Type::Path(tp) if !tp.path.segments.is_empty() && tp.path.segments.last().unwrap().ident == "BTreeMap" => {
            let seg = tp.path.segments.last().unwrap();
            if let PathArguments::AngleBracketed(args) = &seg.arguments {
                let mut iter = args.args.iter();
                let key_ty = match iter.next().expect("BTreeMap needs key").clone() {
                    syn::GenericArgument::Type(t) => t,
                    _ => panic!("Unsupported BTreeMap key arg"),
                };
                let val_ty = match iter.next().expect("BTreeMap needs val").clone() {
                    syn::GenericArgument::Type(t) => t,
                    _ => panic!("Unsupported BTreeMap val arg"),
                };

                let k_ident = format_ident!("__k_{}", depth);
                let v_ident = format_ident!("__v_{}", depth);
                let key_code = gen_deserialize_code(&key_ty, quote!(#k_ident), depth + 1);
                let val_code = gen_deserialize_code(&val_ty, quote!(#v_ident), depth + 1);

                quote! {
                    let mut #target = ::std::collections::BTreeMap::new();
                    if _offset + 4 > _length { return Err(::afast::Error::DecodeError); }
                    let mut len_bytes = [0u8; 4];
                    len_bytes.copy_from_slice(&buf[_offset.._offset+4]);
                    _offset += 4;
                    let len = u32::from_be_bytes(len_bytes) as usize;

                    for _ in 0..len {
                        #key_code
                        #val_code
                        #target.insert(#k_ident, #v_ident);
                    }
                }
            } else {
                panic!("Invalid BTreeMap<> args");
            }
        }

        // 单段 Type::Path（基本类型或自定义类型）
        Type::Path(tp) if !tp.path.segments.is_empty() => {
            let last = tp.path.segments.last().unwrap();
            let name = last.ident.to_string();

            match name.as_str() {
                "i8" => quote! { let #target = buf[_offset] as i8; _offset += 1; },
                "u8" => quote! { let #target = buf[_offset]; _offset += 1; },
                "i16" => quote! { if _offset + 2 > _length { return Err(::afast::Error::DecodeError); }; let mut b=[0u8;2]; b.copy_from_slice(&buf[_offset.._offset+2]); _offset+=2; let #target=i16::from_be_bytes(b); },
                "u16" => quote! { if _offset + 2 > _length { return Err(::afast::Error::DecodeError); }; let mut b=[0u8;2]; b.copy_from_slice(&buf[_offset.._offset+2]); _offset+=2; let #target=u16::from_be_bytes(b); },
                "i32" => quote! { if _offset + 4 > _length { return Err(::afast::Error::DecodeError); }; let mut b=[0u8;4]; b.copy_from_slice(&buf[_offset.._offset+4]); _offset+=4; let #target=i32::from_be_bytes(b); },
                "u32" => quote! { if _offset + 4 > _length { return Err(::afast::Error::DecodeError); }; let mut b=[0u8;4]; b.copy_from_slice(&buf[_offset.._offset+4]); _offset+=4; let #target=u32::from_be_bytes(b); },
                "i64" => quote! { if _offset + 8 > _length { return Err(::afast::Error::DecodeError); }; let mut b=[0u8;8]; b.copy_from_slice(&buf[_offset.._offset+8]); _offset+=8; let #target=i64::from_be_bytes(b); },
                "u64" => quote! { if _offset + 8 > _length { return Err(::afast::Error::DecodeError); }; let mut b=[0u8;8]; b.copy_from_slice(&buf[_offset.._offset+8]); _offset+=8; let #target=u64::from_be_bytes(b); },
                "i128" => quote! { if _offset + 16 > _length { return Err(::afast::Error::DecodeError); }; let mut b=[0u8;16]; b.copy_from_slice(&buf[_offset.._offset+16]); _offset+=16; let #target=i128::from_be_bytes(b); },
                "u128" => quote! { if _offset + 16 > _length { return Err(::afast::Error::DecodeError); }; let mut b=[0u8;16]; b.copy_from_slice(&buf[_offset.._offset+16]); _offset+=16; let #target=u128::from_be_bytes(b); },
                "f32" => quote! { if _offset + 4 > _length { return Err(::afast::Error::DecodeError); }; let mut b=[0u8;4]; b.copy_from_slice(&buf[_offset.._offset+4]); _offset+=4; let #target=f32::from_be_bytes(b); },
                "f64" => quote! { if _offset + 8 > _length { return Err(::afast::Error::DecodeError); }; let mut b=[0u8;8]; b.copy_from_slice(&buf[_offset.._offset+8]); _offset+=8; let #target=f64::from_be_bytes(b); },
                "bool" => quote! { if _offset + 1 > _length { return Err(::afast::Error::DecodeError); } let #target = buf[_offset] != 0; _offset += 1; },
                "String" => quote! {
                    if _offset + 4 > _length { return Err(::afast::Error::DecodeError); };
                    let mut len_bytes=[0u8;4];
                    len_bytes.copy_from_slice(&buf[_offset.._offset+4]);
                    _offset +=4;
                    let len = u32::from_be_bytes(len_bytes) as usize;
                    if _offset + len > _length { return Err(::afast::Error::DecodeError); };
                    let #target = String::from_utf8(buf[_offset.._offset+len].to_vec()).unwrap();
                    _offset += len;
                },
                // 自定义类型（例如 Request2 等）
                _ => {
                    let ty_tokens = &tp.path;
                    quote! {
                        let _sub_bytes = &buf[_offset..];
                        let (#target, _len) = <#ty_tokens as ::afast::AFastData>::from_bytes(_sub_bytes)?;
                        _offset += _len;
                    }
                }
            }
        }

        // 其它未支持的类型
        _ => panic!("Unsupported type in gen_deserialize_code: {:?}", stringify!(ty)),
    }
}
