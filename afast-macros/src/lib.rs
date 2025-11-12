use proc_macro::TokenStream;

mod atd;
mod deserialize;
mod handler;
mod serialize;

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

use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, LitInt, LitStr, Meta, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned as _,
};

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
            fn kind() -> Kind {
                #kind_impl
            }

            fn field(name: &str) -> Field {
                Field { name: name.to_string(), kind: <#name as AFastKind>::kind(), tag: None }
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
                    let field_name = f.ident.as_ref().unwrap();
                    let field_type = &f.ty;
                    let kind_expr = type_to_kind_expr(field_type);
                    let field_tag = parse_tags(&f.attrs)?;
                    let tag_expr = generate_tag_expression(&field_tag);

                    field_kinds.push(quote! {
                        Field { name: stringify!(#field_name).to_string(), kind: #kind_expr, tag: #tag_expr }
                    });
                }

                Ok(quote! { Kind::Struct { fields: vec![#(#field_kinds),*] } })
            }
            syn::Fields::Unnamed(fields) => {
                let mut field_kinds = Vec::new();
                for (i, f) in fields.unnamed.iter().enumerate() {
                    let field_type = &f.ty;
                    let kind_expr = type_to_kind_expr(field_type);
                    let field_tag = parse_tags(&f.attrs)?;
                    let tag_expr = generate_tag_expression(&field_tag);

                    field_kinds.push(quote! {
                        Field { name: format!("_{}", #i), kind: #kind_expr, tag: #tag_expr }
                    });
                }
                Ok(quote! { Kind::Struct { fields: vec![#(#field_kinds),*] } })
            }
            syn::Fields::Unit => Ok(quote! { Kind::Struct { fields: vec![] } }),
        },
        Data::Enum(data_enum) => {
            let mut variant_kinds = Vec::new();
            for variant in &data_enum.variants {
                let variant_kind = match &variant.fields {
                    syn::Fields::Unit => quote! { Kind::Unit },
                    syn::Fields::Unnamed(fields) => {
                        let mut field_kinds = Vec::new();
                        for (i, f) in fields.unnamed.iter().enumerate() {
                            let field_type = &f.ty;
                            let kind_expr = type_to_kind_expr(field_type);
                            let field_tag = parse_tags(&f.attrs)?;
                            let tag_expr = generate_tag_expression(&field_tag);

                            field_kinds.push(quote! { Field { name: format!("_{}", #i), kind: #kind_expr, tag: #tag_expr } });
                        }
                        quote! { Kind::Struct { fields: vec![#(#field_kinds),*] } }
                    }
                    syn::Fields::Named(fields) => {
                        let mut field_kinds = Vec::new();
                        for f in &fields.named {
                            let field_name = f.ident.as_ref().unwrap();
                            let field_type = &f.ty;
                            let kind_expr = type_to_kind_expr(field_type);
                            let field_tag = parse_tags(&f.attrs)?;
                            let tag_expr = generate_tag_expression(&field_tag);

                            field_kinds.push(quote! { Field { name: stringify!(#field_name).to_string(), kind: #kind_expr, tag: #tag_expr } });
                        }
                        quote! { Kind::Struct { fields: vec![#(#field_kinds),*] } }
                    }
                };
                variant_kinds.push(variant_kind);
            }
            Ok(quote! { Kind::Enum { variants: vec![#(#variant_kinds),*] } })
        }
        Data::Union(_) => Ok(quote! { Kind::Struct { fields: vec![] } }),
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
        quote! { Some(Tag { name: #name, description: #description, required: #required, min: #min, max: #max }) }
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
                        } else if meta.path.is_ident("description") {
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
                            return quote! { Kind::Nullable(Box::new(#inner_kind)) };
                        }
                    } else if type_name == "Vec" {
                        if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                            let inner_kind = type_to_kind_expr(inner_ty);
                            return quote! { Kind::Vec(Box::new(#inner_kind)) };
                        }
                    }
                }
                match type_name.as_str() {
                    "i8" => quote! { Kind::I8 },
                    "i16" => quote! { Kind::I16 },
                    "i32" => quote! { Kind::I32 },
                    "i64" => quote! { Kind::I64 },
                    "i128" => quote! { Kind::I128 },
                    "u8" => quote! { Kind::U8 },
                    "u16" => quote! { Kind::U16 },
                    "u32" => quote! { Kind::U32 },
                    "u64" => quote! { Kind::U64 },
                    "u128" => quote! { Kind::U128 },
                    "f32" => quote! { Kind::F32 },
                    "f64" => quote! { Kind::F64 },
                    "bool" => quote! { Kind::Bool },
                    "String" => quote! { Kind::String },
                    "str" => quote! { Kind::String },
                    "()" => quote! { Kind::Unit },
                    _ => quote! { <#ty as AFastKind>::kind() },
                }
            } else {
                quote! { Kind::String }
            }
        }
        syn::Type::Tuple(type_tuple) => {
            if type_tuple.elems.is_empty() {
                quote! { Kind::Unit }
            } else {
                quote! { Kind::String }
            }
        }
        _ => quote! { Kind::String },
    }
}
