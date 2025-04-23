use convert_case::{Case, Casing as _};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{
    Expr, ExprLit, FnArg, Ident, ItemFn, Lit, Meta, MetaNameValue, Pat, PatType, ReturnType, Token,
    Type, parse_quote,
};

#[derive(FromMeta)]
pub struct ToolArgs {
    description: Option<String>,
}

pub fn generate(args: ToolArgs, item: ItemFn) -> TokenStream {
    let tool_name = &item.sig.ident;
    let tool_struct_name = format_ident!("{}Tool", tool_name.to_string().to_case(Case::Pascal));

    // Use description from args if provided, otherwise extract from doc comments
    let description = if let Some(desc) = args.description {
        desc
    } else {
        // Only process doc comments if no description arg is provided
        let doc_description = item
            .attrs
            .iter()
            .filter_map(|attr| {
                if !attr.path().is_ident("doc") {
                    return None;
                }
                let Meta::NameValue(MetaNameValue {
                    value:
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(doc), ..
                        }),
                    ..
                }) = &attr.meta
                else {
                    panic!("Expected a doc attribute but got {:?}", attr);
                };
                Some(doc.value())
            })
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();

        if doc_description.is_empty() {
            return quote! {
                compile_error!("Tool must have a description. Either provide a description argument to the #[tool] attribute or add doc comments to the function.");
            };
        }

        doc_description
    };

    let InputSchema {
        struct_def: input_struct_def,
        struct_type: input_struct_type,
        get_schema,
        destructure_input,
        call,
    } = match generate_input_schema(tool_name, &item) {
        Ok(input_schema) => input_schema,
        Err(e) => return e,
    };

    let output_type = match &item.sig.output {
        ReturnType::Type(_arrow, ty) => ty.clone(),
        ReturnType::Default => {
            parse_quote!(())
        }
    };

    let call = if item.sig.asyncness.is_some() {
        quote!(#call.await)
    } else {
        quote!(#call)
    };

    quote! {
        #item
        #input_struct_def

        #[derive(Default)]
        pub struct #tool_struct_name;

        ::mmcp::server::inventory::submit! { ::mmcp::server::inventory::ToolRegistration::new::<#tool_struct_name>() }

        impl ::mmcp::server::primitives::tool::TypedTool for #tool_struct_name {
            type Input = #input_struct_type;
            type Output = #output_type;

            fn name(&self) -> std::borrow::Cow<'static, str> {
                stringify!(#tool_name).into()
            }

            fn description(&self) -> std::borrow::Cow<'static, str> {
                #description.into()
            }

            fn input_schema(&self) -> std::borrow::Cow<'static, str> {
                #get_schema.into()
            }

            fn annotations(&self) -> ::mmcp::protocol::mcp::ToolAnnotations {
                ::mmcp::protocol::mcp::ToolAnnotations::default()
            }

            async fn execute(&self, arguments: Self::Input) -> Self::Output {
                #destructure_input
                #call
            }
        }
    }
}

struct InputSchema {
    /// The definition of the struct. May be empty if the tool does not take any arguments.
    struct_def: TokenStream,
    /// The name of the struct.
    struct_type: TokenStream,
    /// Expression to get json schema string for the input of the tool.
    get_schema: TokenStream,
    /// Expression to destructure the input of the tool.
    destructure_input: TokenStream,
    /// Expression to call the tool.
    call: TokenStream,
}

pub struct Field<'a> {
    ident: &'a Ident,
    colon_token: &'a Token![:],
    ty: &'a Type,
}

impl ToTokens for Field<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            ident,
            colon_token,
            ty,
        } = self;
        tokens.extend(quote! {
            #ident #colon_token #ty
        });
    }
}

fn generate_input_schema(tool_name: &Ident, input: &ItemFn) -> Result<InputSchema, TokenStream> {
    if input.sig.inputs.is_empty() {
        return Ok(InputSchema {
            struct_def: TokenStream::new(),
            struct_type: parse_quote!(::mmcp::schemars::Any),
            get_schema: quote! {
                r#"{"type": "object"}"#
            },
            destructure_input: quote! {},
            call: quote! {
                #tool_name()
            },
        });
    }

    let struct_name = format_ident!("{}InputSchema", tool_name.to_string().to_case(Case::Pascal));

    let fields = input
        .sig
        .inputs
        .iter()
        .map(|input| {
            let FnArg::Typed(PatType {
                pat,
                colon_token,
                ty,
                ..
            }) = input
            else {
                return Err(quote! {
                    compile_error!("Expected a typed argument but got {}", stringify!(#input))
                });
            };
            let Pat::Ident(ident) = &**pat else {
                return Err(quote! {
                    compile_error!("Expected an ident pattern but got {}", stringify!(#input))
                });
            };

            Ok(Field {
                ident: &ident.ident,
                colon_token,
                ty,
            })
        })
        .collect::<Result<Vec<_>, TokenStream>>()?;

    let field_names = fields.iter().map(|field| &field.ident).collect::<Vec<_>>();

    let struct_def = quote! {
        #[derive(Debug, Clone, PartialEq, ::mmcp::serde::Serialize, ::mmcp::serde::Deserialize, ::mmcp::schemars::JsonSchema)]
        #[serde(crate = "::mmcp::serde")]
        #[schemars(crate = "::mmcp::schemars")]
        pub struct #struct_name {
            #(#fields,)*
        }
    };

    let get_schema = quote! {
        ::mmcp::serde_json::to_string(&::mmcp::schemars::schema_for!(#struct_name)).expect("Failed to serialize schema with serde_json")
    };

    let destructure_input = quote! {
        let #struct_name { #(#field_names,)* } = arguments;
    };

    let call = quote! {
        #tool_name(#(#field_names),*)
    };

    Ok(InputSchema {
        struct_def,
        struct_type: parse_quote!(#struct_name),
        get_schema,
        destructure_input,
        call,
    })
}
