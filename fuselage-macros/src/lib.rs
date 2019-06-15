extern crate proc_macro;
use syn::{parse_macro_input};
use quote::quote;

#[proc_macro_attribute]
pub fn sum(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = proc_macro2::TokenStream::from(args);
    let input = proc_macro2::TokenStream::from(input);

    input.into()
}
