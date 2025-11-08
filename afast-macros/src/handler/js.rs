use proc_macro2::TokenStream as TS;
use quote::quote;
use syn::Error;

use crate::{deserialize, parse_tags, serialize};

pub fn handler_struct(
    _name: &syn::Ident,
    data: &syn::DataStruct,
) -> Result<(Vec<TS>, Vec<TS>, Vec<TS>, Vec<TS>), Error> {
    let mut serialize_js_code = Vec::new();
    let mut serialize_js_type = Vec::new();
    let mut serialize_js_validate = Vec::new();
    let mut deserialize_js_code = Vec::new();

    serialize_js_type.push(quote! {code.push_str("{");});

    match &data.fields {
        syn::Fields::Named(fields_named) => {
            for field in fields_named.named.iter() {
                let ident = &field.ident;
                let ty = &field.ty;
                let typ = quote!(#ty).to_string();
                let tag = parse_tags(&field.attrs)?;
                serialize_js_code.push(serialize::gen_js_code::gen_js_code(
                    &typ,
                    &quote!(#ident),
                    &quote!(request),
                    0,
                    &ty,
                ));
                let field_name = ident.clone().unwrap().to_string();
                serialize_js_type.push(quote! {
                    code.push_str(&format!("{}:", #field_name));
                });
                serialize_js_type.push(serialize::gen_js_type::gen_js_type(
                    &typ,
                    &quote!(#ident),
                    &quote!(request),
                    0,
                    &ty,
                ));
                serialize_js_type.push(quote! {
                    code.push_str(";");
                });
                deserialize_js_code.push(quote! {
                    code.push_str(&format!("{}:", #field_name));
                });
                deserialize_js_code.push(deserialize::gen_js_code::gen_js_code(
                    &typ,
                    &quote!(#ident),
                    &quote!(request),
                    0,
                    &ty,
                ));
                deserialize_js_code.push(quote! {
                    code.push_str(",");
                });
                let field_name = ident.clone().unwrap().to_string();
                let typ = typ.replace(" ", "");
                if typ.starts_with("Vec") {
                    if let Some(msg) = tag.required {
                        serialize_js_validate.push(quote! {
                            code.push_str(&format!("if ({}.{}.length === 0) throw new AFastValidateError('{}');", prefix, #field_name, #msg));
                        });
                    }
                    if let Some((min, msg)) = tag.min {
                        serialize_js_validate.push(quote! {
                            code.push_str(&format!("if ({}.{}.length < {}) throw new AFastValidateError('{}');", prefix, #field_name, #min, #msg));
                        });
                    }
                    if let Some((max, msg)) = tag.max {
                        serialize_js_validate.push(quote! {
                            code.push_str(&format!("if ({}.{}.length > {}) throw new AFastValidateError('{}');", prefix, #field_name, #max, #msg));
                        });
                    }
                } else if typ.starts_with("Option") {
                } else {
                    match typ.as_str() {
                        "String" => {
                            if let Some(msg) = tag.required {
                                serialize_js_validate.push(quote! {
                                    code.push_str(&format!("if ({}.{}.length === 0) throw new AFastValidateError('{}');", prefix, #field_name, #msg));
                                });
                            }
                        }
                        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32"
                        | "u64" | "u128" | "usize" | "f32" | "f64" => {
                            if let Some(msg) = tag.required {
                                serialize_js_validate.push(quote! {
                                    code.push_str(&format!("if ({}.{} === 0) throw new AFastValidateError('{}');", prefix, #field_name, #msg));
                                });
                            }
                            if let Some((min, msg)) = tag.min {
                                serialize_js_validate.push(quote! {
                                    code.push_str(&format!("if ({}.{} < {}) throw new AFastValidateError('{}');", prefix, #field_name, #min, #msg));
                                });
                            }
                            if let Some((max, msg)) = tag.max {
                                serialize_js_validate.push(quote! {
                                    code.push_str(&format!("if ({}.{} > {}) throw new AFastValidateError('{}');", prefix, #field_name, #max, #msg));
                                });
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        syn::Fields::Unnamed(_fields_unnamed) => {}
        syn::Fields::Unit => {}
    }

    serialize_js_type.push(quote! {code.push_str("}");});

    Ok((
        serialize_js_code,
        serialize_js_validate,
        serialize_js_type,
        deserialize_js_code,
    ))
}

pub fn handler_enum(
    _name: &syn::Ident,
    data: &syn::DataEnum,
) -> Result<(Vec<TS>, Vec<TS>, Vec<TS>, Vec<TS>), Error> {
    let mut serialize_js_code = Vec::new();
    let mut serialize_js_type = Vec::new();
    let mut deserialize_js_code = Vec::new();

    let count = data.variants.len();

    for (idx, variant) in data.variants.iter().enumerate() {
        let index = idx as u32;
        match &variant.fields {
            syn::Fields::Named(fields_named) => {
                let mut field_idents = Vec::new();
                let mut ser_fields = Vec::new();
                let mut deser_fields = Vec::new();
                let mut ser_js = Vec::new();
                let mut ser_js_type = Vec::new();
                let mut deser_js = Vec::new();
                for field in fields_named.named.iter() {
                    ser_js_type.push(quote! {
                        code.push_str(&format!("_type:{};", #idx));
                    });
                    let ident = &field.ident;
                    field_idents.push(ident);
                    let ty = &field.ty;
                    let typ = quote!(#ty).to_string();
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
                    ser_js.push(serialize::gen_js_code::gen_js_code(
                        &typ,
                        &quote!(#ident),
                        &quote!(request),
                        0,
                        &ty,
                    ));
                    let field_name = ident.clone().unwrap().to_string();
                    ser_js_type.push(quote! {
                        code.push_str(&format!("{}:", #field_name));
                    });
                    ser_js_type.push(serialize::gen_js_type::gen_js_type(
                        &typ,
                        &quote!(#ident),
                        &quote!(request),
                        0,
                        &ty,
                    ));
                    ser_js_type.push(quote! {
                        code.push_str(";");
                    });
                    deser_js.push(quote! {
                        code.push_str(&format!("{}:", #field_name));
                    });
                    deser_js.push(deserialize::gen_js_code::gen_js_code(
                        &typ,
                        &quote!(#ident),
                        &quote!(request),
                        0,
                        &ty,
                    ));
                    deser_js.push(quote! {
                        code.push_str(",");
                    });
                }

                serialize_js_type.push(quote! {
                    code.push_str("{");
                    #( #ser_js_type )*
                    code.push_str("}");
                });

                if idx < count - 1 {
                    serialize_js_type.push(quote! {
                        code.push_str("|");
                    });
                }

                serialize_js_code.push(quote! {
                    code.push_str(&format!("case {}:", #index));
                    #( #ser_js )*
                    code.push_str("break;");
                });

                deserialize_js_code.push(quote! {
                    code.push_str(&format!("case {}: return {{", #index));
                    #( #deser_js )*
                    code.push_str("};");
                });
            }
            syn::Fields::Unnamed(_fields_unnamed) => {}
            syn::Fields::Unit => {}
        }
    }

    Ok((
        vec![quote! {
            code.push_str(&format!("_b1.pU32({}._type);switch ({}._type) {{", prefix, prefix));
            #( #serialize_js_code )*
            code.push_str("default:throw new Error('unknown variant');}");
        }],
        vec![],
        serialize_js_type,
        vec![quote! {
            code.push_str("...(function(){switch (_b2.rU32()) {");
            #( #deserialize_js_code )*
            code.push_str("default:throw new Error('unknown variant');}}())");
        }],
    ))
}
