use crate::AFastData;
use crate::AFastKind;
use crate::HandlerGeneric;

use super::Field;
use super::Kind;
use super::Tag;

impl Tag {
    pub fn gen_doc(&self) -> String {
        let mut parts = Vec::new();

        if let Some(name) = &self.name {
            parts.push(format!(r#""name":"{}""#, name));
        }
        if let Some(description) = &self.description {
            parts.push(format!(r#""description":"{}""#, description));
        }
        if let Some(required) = &self.required {
            parts.push(format!(r#""required":"{}""#, required));
        }
        if let Some((min, unit)) = &self.min {
            parts.push(format!(r#""min":{{"value":{},"unit":"{}"}}"#, min, unit));
        }
        if let Some((max, unit)) = &self.max {
            parts.push(format!(r#""max":{{"value":{},"unit":"{}"}}"#, max, unit));
        }

        format!("{{{}}}", parts.join(","))
    }
}

impl Field {
    pub fn gen_doc(&self) -> String {

        let kind_doc = self.kind.gen_doc();
        let tag_doc = if let Some(tag) = &self.tag {
            tag.gen_doc()
        } else {
            "null".to_string()
        };
        format!(r#"{{"name":"{}","tag":{},{}}}"#, self.name, tag_doc, kind_doc)
    }
}

impl Kind {
    pub fn gen_doc(&self) -> String {
        let (nullable, inner_doc) = self.gen_doc_inner();
        if nullable {
            format!(r#""nullable":true,{}"#, inner_doc)
        } else {
            inner_doc
        }
    }

    fn gen_doc_inner(&self) -> (bool, String) {
        match self {
            Kind::Unit => (false, r#""kind":"unit""#.to_string()),
            Kind::I8 => (false, r#""kind":"number""#.to_string()),
            Kind::I16 => (false, r#""kind":"number""#.to_string()),
            Kind::I32 => (false, r#""kind":"number""#.to_string()),
            Kind::I64 => (false, r#""kind":"number""#.to_string()),
            Kind::I128 => (false, r#""kind":"number""#.to_string()),
            Kind::U8 => (false, r#""kind":"number""#.to_string()),
            Kind::U16 => (false, r#""kind":"number""#.to_string()),
            Kind::U32 => (false, r#""kind":"number""#.to_string()),
            Kind::U64 => (false, r#""kind":"number""#.to_string()),
            Kind::U128 => (false, r#""kind":"number""#.to_string()),
            Kind::F32 => (false, r#""kind":"number""#.to_string()),
            Kind::F64 => (false, r#""kind":"number""#.to_string()),
            Kind::Bool => (false, r#""kind":"boolean""#.to_string()),
            Kind::String => (false, r#""kind":"string""#.to_string()),
            Kind::Vec(kind) => {
                let (_, inner_doc) = kind.gen_doc_inner();
                (
                    false,
                    format!(r#""kind":"array","items":{{{}}}"#, inner_doc),
                )
            }
            Kind::Enum { variants } => {
                let variants_doc = variants
                    .iter()
                    .enumerate()
                    .map(|(i, v)| {
                        let (_, v_doc) = v.gen_doc_inner();
                        format!(r#"{{"name":"{}",{}}}"#, i, v_doc)
                    })
                    .collect::<Vec<_>>();
                (
                    false,
                    format!(r#""kind":"enum","variants":[{}]"#, variants_doc.join(",")),
                )
            }
            Kind::Struct { fields } => {
                let fields_doc = fields.iter().map(|f| f.gen_doc()).collect::<Vec<_>>();
                (
                    false,
                    format!(r#""kind":"object","fields":[{}]"#, fields_doc.join(",")),
                )
            }
            Kind::Tuple(elements) => {
                let items_doc = elements
                    .iter()
                    .enumerate()
                    .map(|(i, k)| {
                        let (_, inner_doc) = k.gen_doc_inner();
                        format!(r#"{{"index":{},"item":{{{}}}}}"#, i, inner_doc)
                    })
                    .collect::<Vec<_>>();
                (
                    false,
                    format!(r#""kind":"tuple","items":[{}]"#, items_doc.join(",")),
                )
            }
            Kind::Nullable(kind) => {
                let (_, inner_doc) = kind.gen_doc_inner();
                (true, inner_doc)
            }
        }
    }
}

pub fn gen_doc<T, H>(_index: usize, handlers: &Vec<HandlerGeneric<T, H>>) -> String
where
    T: Clone + Send + Sync + 'static,
    H: AFastKind + AFastData,
{
    format!(
        "[{}]",
        handlers
            .iter()
            .map(|h| h.doc.clone())
            .collect::<Vec<String>>()
            .join(",")
    )
}
