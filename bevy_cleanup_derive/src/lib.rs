#![warn(clippy::all)]
#![warn(missing_docs)]

//! Derive macros for `bevy_cleanup`.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Automatically implements the [`bevy_cleanup::Cleanup`] trait for a type. You must also derive [`Component`].
/// 
/// This will simply make an empty impl block for the type, since Cleanup is just a marker trait.
#[proc_macro_derive(Cleanup)]
pub fn derive_cleanup(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = input.generics;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics bevy_cleanup::Cleanup for #name #type_generics #where_clause {}
    })
}
