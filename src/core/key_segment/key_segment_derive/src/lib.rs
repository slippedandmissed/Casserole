use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(KeySegment)]
pub fn key_segment_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_key_segment(&ast)
}


fn impl_key_segment(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl KeySegment for #name {
            fn key_segment(&self) -> String {
                return stringify!(#name).to_string();
            }
        }
    };
    gen.into()
}
