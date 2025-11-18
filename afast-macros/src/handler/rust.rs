use proc_macro2::TokenStream as TS;
use quote::quote;
use syn::{Error, spanned::Spanned as _};

use crate::{deserialize, parse_ignore, parse_tags, serialize};

pub fn handler_struct(
    _name: &syn::Ident,
    data: &syn::DataStruct,
) -> Result<(Vec<TS>, Vec<TS>, Vec<TS>), Error> {
    let mut serialize_code = Vec::new();
    let mut deserialize_code = Vec::new();
    let mut valideate_code = Vec::new();
    let mut field_inits = Vec::new();

    match &data.fields {
        syn::Fields::Named(fields_named) => {
            for field in fields_named.named.iter() {
                let ident = &field.ident;
                field_inits.push(ident);
                let ty = &field.ty;
                let typ = quote!(#ty).to_string();
                let tag = parse_tags(&field.attrs)?;
                if parse_ignore(&field.attrs)? {
                    deserialize_code.push(quote! {
                        let #ident = #ty::default();
                    });
                } else {
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
                }
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
                        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32"
                        | "u64" | "u128" | "usize" | "f32" | "f64" => {
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
                        "bool" => {}
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

    deserialize_code.push(quote! {
        Ok((Self { #(#field_inits:#field_inits,)* }, _offset))
    });

    Ok((serialize_code, deserialize_code, valideate_code))
}

pub fn handler_enum(
    _name: &syn::Ident,
    data: &syn::DataEnum,
) -> Result<(Vec<TS>, Vec<TS>, Vec<TS>), Error> {
    let mut serialize_code = Vec::new();
    let mut deserialize_code = Vec::new();
    let mut valideate_code = Vec::new();

    for (idx, variant) in data.variants.iter().enumerate() {
        let index = idx as u32;
        let variant_ident = &variant.ident;
        match &variant.fields {
            syn::Fields::Named(fields_named) => {
                let mut field_idents = Vec::new();
                let mut ser_fields = Vec::new();
                let mut deser_fields = Vec::new();
                for field in fields_named.named.iter() {
                    let ident = &field.ident;
                    field_idents.push(ident);
                    let ty = &field.ty;
                    let typ = quote!(#ty).to_string();
                    let tag = parse_tags(&field.attrs)?;
                    if parse_ignore(&field.attrs)? {
                        deser_fields.push(quote! {
                            let #ident = #ty::default();
                        });
                    } else {
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
                    }
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

                deserialize_code.push(quote! {
                    #index => {
                        #( #deser_fields )*
                        Ok((Self::#variant_ident { #( #field_idents ),* }, _offset))
                    }
                });
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                let mut field_idents = Vec::new();
                let mut ser_fields = Vec::new();
                let mut deser_fields = Vec::new();

                for (i, field) in fields_unnamed.unnamed.iter().enumerate() {
                    let ident = syn::Ident::new(&format!("_{}", i), field.span());
                    field_idents.push(ident.clone());

                    let ty = &field.ty;
                    let typ = quote!(#ty).to_string().replace(" ", "");
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
                            _ => {
                                valideate_code.push(quote! {
                                    self.#ident.validate()?;
                                });
                            }
                        }
                    }
                }

                serialize_code.push(quote! {
                    Self::#variant_ident(#( #field_idents ),*) => {
                        buf.extend(&#index.to_be_bytes());
                        #( #ser_fields )*
                    }
                });

                deserialize_code.push(quote! {
                    #index => {
                        #( #deser_fields )*
                        Ok((Self::#variant_ident(#( #field_idents ),*), _offset))
                    }
                });
            }
            syn::Fields::Unit => {
                serialize_code.push(quote! {
                    Self::#variant_ident => {
                        buf.extend(&#index.to_be_bytes());
                    }
                });
                deserialize_code.push(quote! {
                    #index => {
                        Ok((Self::#variant_ident, _offset))
                    }
                });
            }
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
        valideate_code,
    ))
}
