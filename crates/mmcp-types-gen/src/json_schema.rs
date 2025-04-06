use indexmap::IndexMap;
use serde::Deserialize;

use crate::type_registry::TypeRef;

#[derive(Deserialize, Default, Clone)]
pub struct RootSchema {
    pub definitions: IndexMap<String, Schema>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum Schema {
    TypeTagged(TypeTaggedSchema),
    TypeEnum(TypeEnumSchema),
    AnyOf(AnyOfSchema),
    Ref(RefSchema),
    Enum(EnumSchema),
    Empty(EmptySchema),
}

impl Schema {
    pub(crate) fn get_description(&self) -> Option<String> {
        match self {
            Self::TypeTagged(TypeTaggedSchema::Object(object)) => object.shared.description.clone(),
            Self::TypeTagged(TypeTaggedSchema::Array(array)) => array.shared.description.clone(),
            Self::TypeTagged(TypeTaggedSchema::Number(number)) => number.shared.description.clone(),
            Self::TypeTagged(TypeTaggedSchema::Integer(integer)) => {
                integer.shared.description.clone()
            }
            Self::TypeTagged(TypeTaggedSchema::String(string)) => string.get_description(),
            Self::TypeTagged(TypeTaggedSchema::Boolean(boolean)) => {
                boolean.shared.description.clone()
            }
            Self::TypeEnum(type_enum) => type_enum.shared.description.clone(),
            Self::AnyOf(any_of) => any_of.shared.description.clone(),
            Self::Ref(ref_schema) => ref_schema.shared.description.clone(),
            Self::Enum(enum_schema) => enum_schema.shared.description.clone(),
            Self::Empty(empty_schema) => empty_schema.shared.description.clone(),
        }
    }

    pub fn type_ref(&self, root_schema: &RootSchema) -> Option<TypeRef> {
        match self {
            Schema::TypeTagged(TypeTaggedSchema::Number(_)) => return Some(TypeRef::Number),
            Schema::TypeTagged(TypeTaggedSchema::Integer(_)) => return Some(TypeRef::Integer),
            Schema::TypeTagged(TypeTaggedSchema::String(string)) => {
                if let Some(type_ref) = string.type_ref() {
                    return Some(type_ref);
                }
            }
            Schema::TypeTagged(TypeTaggedSchema::Boolean(_)) => return Some(TypeRef::Boolean),
            Schema::Ref(ref_schema) => {
                return Some(TypeRef::Ref(extract_ref_name(&ref_schema.r#ref)));
            }
            Schema::Empty(_) => return Some(TypeRef::Any),
            Schema::TypeTagged(TypeTaggedSchema::Object(object)) => {
                if let Some(type_ref) = object.wildcard_type_ref(root_schema) {
                    return Some(type_ref);
                }
            }
            _ => {}
        }
        root_schema
            .definitions
            .iter()
            .find(|(_, s)| *s == self)
            .map(|(name, _)| TypeRef::Ref(name.clone()))
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EmptySchema {
    #[serde(flatten)]
    pub shared: SharedSchema,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum TypeTaggedSchema {
    Object(ObjectSchema),
    Array(ArraySchema),
    Number(NumberSchema),
    Integer(IntegerSchema),
    String(StringSchema),
    Boolean(BooleanSchema),
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TypeEnumSchema {
    #[serde(flatten)]
    pub shared: SharedSchema,
    pub r#type: Vec<EnumType>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum EnumType {
    String,
    Number,
    Integer,
}

impl std::fmt::Display for EnumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String => write!(f, "String"),
            Self::Number => write!(f, "Number"),
            Self::Integer => write!(f, "Integer"),
        }
    }
}

impl EnumType {
    pub fn type_ref(&self) -> TypeRef {
        match self {
            Self::String => TypeRef::String,
            Self::Number => TypeRef::Number,
            Self::Integer => TypeRef::Integer,
        }
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AnyOfSchema {
    #[serde(flatten)]
    pub shared: SharedSchema,
    pub any_of: Vec<Schema>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RefSchema {
    #[serde(flatten)]
    pub shared: SharedSchema,
    #[serde(rename = "$ref")]
    pub r#ref: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SharedSchema {
    #[serde(default)]
    pub description: Option<String>,
}

// We also consider one schema to be equal to another if one of them have None description.
impl PartialEq for SharedSchema {
    fn eq(&self, other: &Self) -> bool {
        match (&self.description, &other.description) {
            (Some(a), Some(b)) => a == b,
            _ => true,
        }
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ObjectSchema {
    #[serde(flatten)]
    pub shared: SharedSchema,
    #[serde(default)]
    pub properties: IndexMap<String, Schema>,
    #[serde(default)]
    pub required: Vec<String>,
    #[serde(default)]
    pub additional_properties: AdditionalProperties,
}
impl ObjectSchema {
    fn wildcard_type_ref(&self, root_schema: &RootSchema) -> Option<TypeRef> {
        if self.properties.is_empty() && self.required.is_empty() {
            self.additional_properties.wildcard_type_ref(root_schema)
        } else {
            None
        }
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged, rename_all = "camelCase")]
pub enum AdditionalProperties {
    Bool(bool),
    Schema(Box<Schema>),
}
impl AdditionalProperties {
    pub fn wildcard_type_ref(&self, root_schema: &RootSchema) -> Option<TypeRef> {
        match self {
            Self::Bool(true) => Some(TypeRef::AnyObject),
            Self::Schema(schema) => match **schema {
                Schema::Empty(_) => Some(TypeRef::AnyObject),
                _ => schema
                    .type_ref(root_schema)
                    .map(|type_ref| TypeRef::AnyMap(Box::new(type_ref))),
            },
            _ => None,
        }
    }
}

impl Default for AdditionalProperties {
    fn default() -> Self {
        Self::Bool(true)
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArraySchema {
    #[serde(flatten)]
    pub shared: SharedSchema,
    pub items: Box<Schema>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NumberSchema {
    #[serde(flatten)]
    pub shared: SharedSchema,
    #[serde(default)]
    pub maximum: Option<f64>,
    #[serde(default)]
    pub minimum: Option<f64>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IntegerSchema {
    #[serde(flatten)]
    pub shared: SharedSchema,
    #[serde(default)]
    pub maximum: Option<i32>,
    #[serde(default)]
    pub minimum: Option<i32>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged, rename_all = "camelCase")]
pub enum StringSchema {
    Const {
        #[serde(flatten)]
        shared: SharedSchema,
        r#const: String,
    },
    Enum {
        #[serde(flatten)]
        shared: SharedSchema,
        r#enum: Vec<String>,
    },
    Format {
        #[serde(flatten)]
        shared: SharedSchema,
        format: String,
    },
    Plain {
        #[serde(flatten)]
        shared: SharedSchema,
    },
}
impl StringSchema {
    pub(crate) fn type_ref(&self) -> Option<TypeRef> {
        match self {
            Self::Const { r#const, .. } => Some(TypeRef::ConstString(r#const.clone())),
            Self::Enum { .. } => None,
            Self::Format { format, .. } => match format.as_str() {
                "byte" => Some(TypeRef::Bytes),
                _ => Some(TypeRef::String),
            },
            Self::Plain { .. } => Some(TypeRef::String),
        }
    }

    pub fn get_description(&self) -> Option<String> {
        match self {
            Self::Const { shared, .. } => shared.description.clone(),
            Self::Enum { shared, .. } => shared.description.clone(),
            Self::Format { shared, .. } => shared.description.clone(),
            Self::Plain { shared } => shared.description.clone(),
        }
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EnumSchema {
    #[serde(flatten)]
    pub shared: SharedSchema,
    pub r#enum: Vec<String>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BooleanSchema {
    #[serde(flatten)]
    pub shared: SharedSchema,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NullSchema {
    #[serde(flatten)]
    pub shared: SharedSchema,
}

pub fn extract_ref_name(ref_path: &str) -> String {
    // Extract the type name from a reference path like "#/definitions/TypeName"
    ref_path.split('/').next_back().unwrap().to_string()
}
