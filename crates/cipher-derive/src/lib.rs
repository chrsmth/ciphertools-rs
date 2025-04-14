extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(IntoDecipherKey)]
pub fn derive_into_decipher_key(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = input.ident;

  let expanded = quote! {
      impl crate::cipher::IntoDecipherKey for #name {
          type DecipherKey = Self;
          fn into_decipher_key(self) -> Self {
              self
          }
      }
  };

  TokenStream::from(expanded)
}

#[proc_macro_derive(IntoEncipherKey)]
pub fn derive_into_encipher_key(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = input.ident;

  let expanded = quote! {
      impl crate::cipher::IntoEncipherKey for #name {
          type EncipherKey = Self;
          fn into_encipher_key(self) -> Self {
              self
          }
      }
  };

  TokenStream::from(expanded)
}
