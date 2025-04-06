use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct TypeRegistry {
    types: HashMap<String, TypeDef>,
    order: Vec<String>,
}
impl TypeRegistry {
    pub fn register(&mut self, name: String, type_def: TypeDef) {
        self.types.insert(name.clone(), type_def);
        self.order.push(name);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &TypeDef)> {
        self.order
            .iter()
            .filter_map(|name| self.types.get(name).map(|ty| (name, ty)))
    }
}

impl FromIterator<(String, TypeDef)> for TypeRegistry {
    fn from_iter<T: IntoIterator<Item = (String, TypeDef)>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let mut types = HashMap::with_capacity(iter.size_hint().0);
        let mut order = Vec::with_capacity(iter.size_hint().0);

        for (name, type_def) in iter {
            types.insert(name.clone(), type_def);
            order.push(name);
        }

        Self { types, order }
    }
}

#[derive(Clone)]
pub enum TypeRef {
    ConstString(String),
    Ref(String),
    String,
    Bytes,
    Boolean,
    Number,
    Integer,
    Vec(Box<TypeRef>), // A vector of another TypeRef
    /// Matching to any type. Typically serde_json::Value or RawValue
    Any,
    /// Matching to any object type.
    AnyObject,
    /// Matching to any map with string keys and values of chosen type
    AnyMap(Box<TypeRef>),
}

impl TypeRef {
    pub fn variant_name(&self) -> String {
        match self {
            TypeRef::Ref(name) => name.clone(),
            _ => todo!(),
        }
    }

    pub fn to_rust_tokens(&self) -> TokenStream {
        match self {
            TypeRef::ConstString(const_val) => {
                quote! { monostate::MustBe!(#const_val) }
            }
            TypeRef::Ref(name) => {
                let type_ident = format_ident!("{}", name);
                quote! { #type_ident }
            }
            TypeRef::Bytes => quote! {
                Vec<u8>
            },
            TypeRef::String => quote! { String },
            TypeRef::Boolean => quote! { bool },
            TypeRef::Integer => quote! { i64 },
            TypeRef::Number => quote! { f64 },
            TypeRef::Vec(inner) => {
                let inner_tokens = inner.to_rust_tokens();
                quote! { Vec<#inner_tokens> }
            }
            TypeRef::Any => quote! { serde_json::Value },
            TypeRef::AnyObject => quote! { serde_json::Map<String, serde_json::Value> },
            TypeRef::AnyMap(inner) => {
                let inner_tokens = inner.to_rust_tokens();
                quote! { indexmap::IndexMap<String, #inner_tokens> }
            }
        }
    }

    pub fn container_attr(&self) -> Option<TokenStream> {
        match self {
            TypeRef::Bytes => Some(quote! { #[serde_with::serde_as] }),
            _ => None,
        }
    }

    pub fn field_attr(&self) -> TokenStream {
        match self {
            TypeRef::Bytes => quote!(#[serde_as(as = "Base64")]),
            _ => quote! {},
        }
    }
}

#[derive(Clone)]
pub enum TypeDef {
    Enum(EnumDef),
    Struct(StructDef),
    NewType(NewTypeDef),
}

#[derive(Clone)]
pub struct EnumDef {
    pub name: String,
    pub description: Option<String>,
    pub variants: Vec<VariantDef>,
    pub untagged: bool,
}

#[derive(Clone)]
pub struct VariantDef {
    pub description: Option<String>,
    pub name: String,
    pub ty: Option<TypeRef>,
    pub rename: Option<String>,
}

#[derive(Clone)]
pub struct StructDef {
    pub name: String,
    pub description: Option<String>,
    pub additional_parameters: Option<TypeRef>,
    pub fields: Vec<FieldDef>,
}

#[derive(Clone)]
pub struct FieldDef {
    pub description: Option<String>,
    pub name: String,
    pub rename: Option<String>,
    pub ty: TypeRef,
    pub required: bool,
}

#[derive(Clone)]
pub struct NewTypeDef {
    pub name: String,
    pub description: Option<String>,
    pub inner_type: TypeRef,
    pub transparent: bool,
}
