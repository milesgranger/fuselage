extern crate proc_macro;
use syn;
use quote::quote;

#[proc_macro_attribute]
pub fn sum(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(args as syn::ExprLit);
    let input = syn::parse_macro_input!(input as syn::ItemFn);

    let fn_name = &input.ident;

    let lit = match &args.lit {
        syn::Lit::Str(s) => s,
        _ => panic!()
    };

    let result = quote! {
        fn #fn_name () {
            println!("{}", #lit);
        }
    };

    result.into()

}
