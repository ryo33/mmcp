use convert_case::{Case, Casing as _};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use crate::{
    json_schema::{
        AnyOfSchema, ArraySchema, EmptySchema, EnumSchema, ObjectSchema, RefSchema, RootSchema,
        Schema, StringSchema, TypeEnumSchema, TypeTaggedSchema, extract_ref_name,
    },
    type_registry::{
        EnumDef, FieldDef, NewTypeDef, StructDef, TypeDef, TypeRef, TypeRegistry, VariantDef,
    },
};

#[derive(Default)]
pub struct TypeGeneratorConfig {
    pub root_schema: RootSchema,
    type_registry: TypeRegistry,
}

impl TypeGeneratorConfig {
    pub fn with_root_schema(mut self, schema: RootSchema) -> Self {
        self.root_schema = schema;
        self
    }

    pub fn generate_type_registry(&mut self) {
        for (name, schema) in &self.root_schema.definitions.clone() {
            let type_def = self.generate_type_def(name, schema);
            self.type_registry.register(name.clone(), type_def);
        }
    }

    fn generate_type_def(&mut self, name: &String, schema: &Schema) -> TypeDef {
        match schema {
            Schema::TypeTagged(type_tagged_schema) => {
                self.handle_type_tagged(name, type_tagged_schema)
            }
            Schema::TypeEnum(type_enum_schema) => self.handle_type_enum(name, type_enum_schema),
            Schema::AnyOf(any_of_schema) => self.handle_any_of(name, any_of_schema),
            Schema::Ref(ref_schema) => self.handle_ref(name, ref_schema),
            Schema::Enum(enum_schema) => self.handle_enum(name, enum_schema),
            Schema::Empty(empty_schema) => self.handle_empty(name, empty_schema),
        }
    }

    fn handle_type_tagged(&mut self, name: &String, schema: &TypeTaggedSchema) -> TypeDef {
        match schema {
            TypeTaggedSchema::Object(obj) => self.handle_object(name, obj),
            TypeTaggedSchema::Array(arr) => self.handle_array(name, arr),
            TypeTaggedSchema::Number(num) => TypeDef::NewType(NewTypeDef {
                name: name.to_string(),
                description: num.shared.description.clone(),
                inner_type: TypeRef::Number,
                transparent: true,
            }),
            TypeTaggedSchema::Integer(int) => TypeDef::NewType(NewTypeDef {
                name: name.to_string(),
                description: int.shared.description.clone(),
                inner_type: TypeRef::Integer,
                transparent: true,
            }),
            TypeTaggedSchema::String(str_schema) => self.handle_string(name, str_schema),
            TypeTaggedSchema::Boolean(bool_schema) => TypeDef::NewType(NewTypeDef {
                name: name.to_string(),
                description: bool_schema.shared.description.clone(),
                inner_type: TypeRef::Boolean,
                transparent: true,
            }),
        }
    }

    fn handle_type_enum(&self, name: &str, schema: &TypeEnumSchema) -> TypeDef {
        TypeDef::Enum(EnumDef {
            name: name.to_string(),
            description: schema.shared.description.clone(),
            variants: schema
                .r#type
                .iter()
                .map(|ty| VariantDef {
                    name: ty.to_string(),
                    description: None,
                    rename: None,
                    ty: Some(ty.type_ref()),
                })
                .collect(),
            untagged: true,
        })
    }

    fn handle_any_of(&mut self, name: &str, schema: &AnyOfSchema) -> TypeDef {
        TypeDef::Enum(EnumDef {
            name: name.to_string(),
            description: schema.shared.description.clone(),
            variants: schema
                .any_of
                .iter()
                .map(|schema| {
                    let Some(type_ref) = schema.type_ref(&self.root_schema) else {
                        panic!("Failed to find type ref for {} {:?}", name, schema);
                    };

                    VariantDef {
                        name: type_ref.variant_name(),
                        description: None,
                        rename: None,
                        ty: Some(type_ref),
                    }
                })
                .collect(),
            untagged: true,
        })
    }

    fn handle_ref(&self, name: &str, schema: &RefSchema) -> TypeDef {
        // For reference schemas, we would typically create a type that references another
        let ref_name = extract_ref_name(&schema.r#ref);
        TypeDef::NewType(NewTypeDef {
            name: name.to_string(),
            description: schema.shared.description.clone(),
            inner_type: TypeRef::Ref(ref_name),
            transparent: true,
        })
    }

    fn handle_enum(&self, name: &str, schema: &EnumSchema) -> TypeDef {
        // Convert enum variants to PascalCase
        let mut variants = Vec::new();

        for variant in &schema.r#enum {
            let (variant_name, rename) = if !variant.is_case(Case::Pascal) {
                let pascal_name = variant.to_case(Case::Pascal);
                (pascal_name, Some(variant.clone()))
            } else {
                (variant.clone(), None)
            };

            variants.push(VariantDef {
                name: variant_name,
                rename,
                description: None,
                ty: None,
            });
        }

        // For enum schemas, create an enum type
        TypeDef::Enum(EnumDef {
            name: name.to_string(),
            description: schema.shared.description.clone(),
            variants,
            untagged: true,
        })
    }

    fn handle_empty(&self, name: &str, schema: &EmptySchema) -> TypeDef {
        // For empty schemas, create a simple struct
        TypeDef::NewType(NewTypeDef {
            name: name.to_string(),
            description: schema.shared.description.clone(),
            inner_type: TypeRef::Any,
            transparent: true,
        })
    }

    fn handle_object(&mut self, name: &str, schema: &ObjectSchema) -> TypeDef {
        let fields = schema
            .properties
            .iter()
            .map(|(field_name, field_schema)| {
                self.handle_object_field(name, schema, field_name, field_schema)
            })
            .collect::<Vec<_>>();

        TypeDef::Struct(StructDef {
            name: name.to_string(),
            description: schema.shared.description.clone(),
            fields,
            additional_parameters: schema
                .additional_properties
                .wildcard_type_ref(&self.root_schema),
        })
    }

    fn handle_object_field(
        &mut self,
        name: &str,
        schema: &ObjectSchema,
        field_name: &String,
        field_schema: &Schema,
    ) -> FieldDef {
        let description = field_schema.get_description();
        let field_type = if let Some(field_type) = field_schema.type_ref(&self.root_schema) {
            field_type
        } else if let Schema::TypeTagged(TypeTaggedSchema::Array(array)) = field_schema {
            let inner = if let Some(inner_type) = array.items.type_ref(&self.root_schema) {
                inner_type
            } else {
                let field_type_name = (name.to_string() + "_" + field_name).to_case(Case::Pascal);
                let type_def = self.generate_type_def(&field_type_name, &array.items);
                self.type_registry
                    .register(field_type_name.clone(), type_def);
                TypeRef::Ref(field_type_name)
            };
            TypeRef::Vec(Box::new(inner))
        } else {
            let field_type_name = (name.to_string() + "_" + field_name).to_case(Case::Pascal);
            let type_def = self.generate_type_def(&field_type_name, field_schema);
            self.type_registry
                .register(field_type_name.clone(), type_def);
            TypeRef::Ref(field_type_name)
        };

        let required = schema.required.contains(field_name);

        // Convert field names to snake case if they're not already
        let (target_name, rename) = if !field_name.is_case(Case::Snake) {
            (field_name.to_case(Case::Snake), Some(field_name.clone()))
        } else {
            (field_name.clone(), None)
        };

        FieldDef {
            name: target_name,
            rename,
            description,
            ty: field_type,
            required,
        }
    }

    fn handle_array(&mut self, name: &str, schema: &ArraySchema) -> TypeDef {
        // For array schemas, create a newtype that wraps a Vec of the item type
        let inner_type = if let Some(inner_type) = schema.items.type_ref(&self.root_schema) {
            inner_type
        } else {
            let item_type_name = format!("{}_Item", name).to_case(Case::Pascal);
            let type_def = self.generate_type_def(&item_type_name, &schema.items);
            self.type_registry
                .register(item_type_name.clone(), type_def);
            TypeRef::Ref(item_type_name)
        };

        // Create a newtype for this array type
        TypeDef::NewType(NewTypeDef {
            name: name.to_string(),
            description: schema.shared.description.clone(),
            inner_type: TypeRef::Vec(Box::new(inner_type)),
            transparent: true,
        })
    }

    fn handle_string(&self, name: &String, schema: &StringSchema) -> TypeDef {
        match schema {
            StringSchema::Enum { shared, r#enum } => self.handle_string_enum(name, shared, r#enum),
            _ => TypeDef::NewType(NewTypeDef {
                name: name.to_string(),
                description: schema.get_description(),
                inner_type: schema
                    .type_ref()
                    .expect("Failed to get type ref for string"),
                transparent: true,
            }),
        }
    }

    fn handle_string_enum(
        &self,
        name: &String,
        shared: &crate::json_schema::SharedSchema,
        r#enum: &Vec<String>,
    ) -> TypeDef {
        // Convert enum variants to PascalCase
        let mut variants = Vec::new();

        for variant in r#enum {
            let (variant_name, rename) = if !variant.is_case(Case::Pascal) {
                let pascal_name = variant.to_case(Case::Pascal);
                (pascal_name, Some(variant.clone()))
            } else {
                (variant.clone(), None)
            };

            variants.push(VariantDef {
                name: variant_name,
                rename,
                description: None,
                ty: None,
            });
        }

        // For enum strings, create an enum type
        TypeDef::Enum(EnumDef {
            name: name.to_string(),
            description: shared.description.clone(),
            variants,
            untagged: false,
        })
    }

    pub fn generate(&mut self) -> TokenStream {
        self.generate_type_registry();

        let definitions = self
            .type_registry
            .iter()
            .map(|(_name, type_def)| match type_def {
                TypeDef::Enum(enum_def) => generate_enum(enum_def),
                TypeDef::Struct(struct_def) => generate_struct(struct_def),
                TypeDef::NewType(newtype_def) => generate_newtype(newtype_def),
            });

        quote! {
            #(#definitions)*
        }
    }
}

fn generate_enum(enum_def: &EnumDef) -> TokenStream {
    let name_ident = quote::format_ident!("{}", enum_def.name);

    let variants_tokens = enum_def.variants.iter().map(|variant| {
        let variant_ident = quote::format_ident!("{}", variant.name);
        let variant_ty = variant.ty.as_ref().map(|ty| {
            let ty = ty.to_rust_tokens();
            quote! { (#ty) }
        });

        let variant_doc = match &variant.description {
            Some(desc) => quote! { #[doc = #desc] },
            None => quote! {},
        };

        let variant_rename = match &variant.rename {
            Some(rename) => quote! { #[serde(rename = #rename)] },
            None => quote! {},
        };

        quote! {
            #variant_doc
            #variant_rename
            #variant_ident #variant_ty,
        }
    });

    let doc_comment = match &enum_def.description {
        Some(desc) => desc.to_string(),
        None => format!(
            "Generated from JSON schema definition for {}",
            enum_def.name
        ),
    };

    let untagged = if enum_def.untagged {
        quote! { #[serde(untagged)] }
    } else {
        quote! {}
    };

    let derive = if enum_def.variants.iter().all(|v| v.ty.is_none()) {
        quote! { #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)] }
    } else {
        quote! { #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)] }
    };

    quote! {
        #[doc = #doc_comment]
        #derive
        #untagged
        pub enum #name_ident {
            #(#variants_tokens)*
        }
    }
}

fn generate_struct(struct_def: &StructDef) -> TokenStream {
    let name_ident = quote::format_ident!("{}", struct_def.name);

    let doc_comment = match &struct_def.description {
        Some(desc) => desc.to_string(),
        None => format!(
            "Generated from JSON schema definition for {}",
            struct_def.name
        ),
    };

    // Check if any field types require special container attributes
    let container_attrs = struct_def
        .fields
        .iter()
        .filter_map(|field| field.ty.container_attr());

    let fields = struct_def.fields.iter().map(|field| {
        let field_name_ident = if is_rust_keyword(&field.name) {
            Ident::new_raw(&field.name, Span::call_site())
        } else {
            Ident::new(&field.name, Span::call_site())
        };

        let field_type = field.ty.to_rust_tokens();

        let field_type = if field.required {
            quote! { #field_type }
        } else {
            quote! { Option<#field_type> }
        };

        let optional_attr = if !field.required {
            quote! { #[serde(default, skip_serializing_if = "Option::is_none")] }
        } else {
            quote! {}
        };

        let field_doc = match &field.description {
            Some(desc) => quote! { #[doc = #desc] },
            None => quote! {},
        };

        // Add rename attribute if needed
        let rename_attr = if let Some(ref original_name) = field.rename {
            quote! { #[serde(rename = #original_name)] }
        } else {
            quote! {}
        };

        // Add any special field attributes
        let field_attr = field.ty.field_attr();

        quote! {
            #field_doc
            #rename_attr
            #optional_attr
            #field_attr
            pub #field_name_ident: #field_type,
        }
    });

    let deny_unknown_fields = if struct_def.additional_parameters.is_none() {
        quote! { #[serde(deny_unknown_fields)] }
    } else {
        quote! {}
    };

    let extra = if let Some(additional_parameters) = &struct_def.additional_parameters {
        let additional_parameters = additional_parameters.to_rust_tokens();
        quote! {
            /// Additional parameters that are not part of the schema.
            #[serde(flatten)]
            pub extra: #additional_parameters,
        }
    } else {
        quote! {}
    };

    quote! {
        #[doc = #doc_comment]
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        #(#container_attrs)*
        #deny_unknown_fields
        pub struct #name_ident {
            #(#fields)*
            #extra
        }
    }
}

fn generate_newtype(newtype_def: &NewTypeDef) -> TokenStream {
    let name_ident = quote::format_ident!("{}", newtype_def.name);

    let doc_comment = match &newtype_def.description {
        Some(desc) => desc.to_string(),
        None => format!(
            "Generated from JSON schema definition for {}",
            newtype_def.name
        ),
    };

    // Define the inner type
    let final_type = newtype_def.inner_type.to_rust_tokens();

    // Add serde(transparent) attribute if needed
    let transparent_attr = if newtype_def.transparent {
        quote! { #[serde(transparent)] }
    } else {
        quote! {}
    };

    // Check if the inner type requires special container attributes
    let container_attr = newtype_def.inner_type.container_attr();

    // Check if the inner type requires special field attributes
    let field_attr = newtype_def.inner_type.field_attr();

    quote! {
        #[doc = #doc_comment]
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        #container_attr
        #transparent_attr
        pub struct #name_ident(
            #field_attr
            pub #final_type
        );
    }
}

// Helper function to check if a string is a Rust keyword. This list is not complete and should be extended when needed.
fn is_rust_keyword(word: &str) -> bool {
    matches!(word, "type" | "ref")
}
