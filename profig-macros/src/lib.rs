use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod derive;

#[proc_macro_derive(Profig, attributes(profig))]
pub fn derive_profig(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive::expand_derive_profig(input).into()
}
