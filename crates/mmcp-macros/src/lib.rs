use darling::{FromMeta as _, ast::NestedMeta};
use proc_macro::TokenStream;

mod tool;

/// Register a function as a tool.
///
/// - For input types, [serde::Deserialize] and [schemars::JsonSchema] are required.
/// - For the output type, [mmcp::server::primitives::tool::IntoToolResult] is required.
/// - You must provide one of `description` argument or doc comments to the tool.
///
/// # Example
///
/// ```rust,ignore
/// /// Description of the tool.
/// #[tool]
/// fn my_tool() -> String {
///     "Hello, world!".to_string()
/// }
///
/// #[tool(description = "Description of the tool.")]
/// fn my_tool() -> String {
///     "Hello, world!".to_string()
/// }
/// ```
#[proc_macro_attribute]
pub fn tool(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let args = match tool::ToolArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };
    let input = syn::parse_macro_input!(input as syn::ItemFn);

    tool::generate(args, input).into()
}
