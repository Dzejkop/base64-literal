extern crate proc_macro;

use base64::Engine;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn base64_literal(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::LitStr);
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(input.value())
        .unwrap();

    let byte_array = quote::quote!([#(#bytes),*]);

    byte_array.into()
}
