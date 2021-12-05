extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use std::any::TypeId;
use syn;

fn impl_component_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Component for #name {
            fn type_id(&self) -> TypeId {
                TypeId::of::<Self>()
            }

                        fn as_any_mut(&mut self) -> &mut dyn Any {
                                        self
                        }

                        fn as_any(&self) -> &dyn Any {
                                self
                        }
        }
    };
    gen.into()
}

#[proc_macro_derive(Component)]
pub fn component_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_component_macro(&ast)
}
