use proc_macro::TokenStream;

mod tool;

#[proc_macro_attribute]
pub fn tool(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemFn);

    tool::generate(input).into()
}
