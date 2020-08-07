extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

// Thanks to this annotation, the following function will be called
// when a user specifies #[derive(HelloMacro)] on a type
#[proc_macro_derive(HelloMacro)]
// Prase the token stream
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    // Transform the syntax tree
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // The quote macro let us define the Rust code we want to return
    // Quote will replace #name with the value of the variable name
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, macro. My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}
