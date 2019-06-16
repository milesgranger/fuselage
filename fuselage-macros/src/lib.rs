extern crate proc_macro;
use syn;
use quote::quote;
use proc_macro2::TokenStream;
use std::str::FromStr;

#[proc_macro_attribute]
pub fn sum(args: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let name = syn::parse_macro_input!(args as syn::LitStr);
    let string_tokens = item.to_string().replace("NAME", &name.value());
    let tokens: proc_macro::TokenStream = TokenStream::from_str(&string_tokens).unwrap().into();

    let input = syn::parse_macro_input!(tokens as syn::ItemFn);

    let fn_name = &input.ident;
    let fn_body = &input.block;
    let fn_return_type = &input.decl.output;
    let fn_inputs = &input.decl.inputs;
    let fn_generics = &input.decl.generics;

    let result = quote! {
        fn #fn_name #fn_generics (#fn_inputs) #fn_return_type {
            println!("{}", #name);
            #fn_body
        }
    };

    result.into()

}
