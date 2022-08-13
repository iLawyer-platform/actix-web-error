extern crate proc_macro;

mod attr;
mod generics;
mod input;
mod json;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Json, attributes(status))]
pub fn derive_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    json::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
