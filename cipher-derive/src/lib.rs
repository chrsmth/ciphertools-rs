#![recursion_limit="1024"]

#[macro_use]
extern crate quote;
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(DictionaryAttack)]
pub fn dictionary_attack(input: TokenStream) -> TokenStream {
	let ast = syn::parse_macro_input!(input as DeriveInput);
	impl_dictionary_attack(&ast)
}

fn impl_dictionary_attack(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;
	let expanded = quote! {
		impl<S> DictionaryAttack<S> for #name where
			S: Iterator<Item = Self::Key>,
		{
			fn dictionary_attack(ciphertext: String, dict: S, n: usize, lang: LanguageModel) -> Vec<Candidate<Self::Key>>
			{
				type Can = Candidate<<#name as Cipher>::Key>;

				let mut candidates = MinMaxHeap::<Can>::new();
				candidates.reserve_exact(n);

				for key in dict {
					let text = #name::decipher(&ciphertext, &key);

					let mut alph_iter = text.chars()
						.map(|x| Alph::try_from(x))
						.filter(|x| x.is_ok())
						.map(|x| x.unwrap());

					let can = Candidate {
						score: lang.score(&mut alph_iter),
						text: text,
						key: key,
					};

					if candidates.len() < candidates.capacity() {
						candidates.push(can);
					} else if *candidates.peek_min().unwrap() < can {
						candidates.replace_min(can);
					}
				}

				candidates.into_vec_desc()
			}
		}
	};
	TokenStream::from(expanded)
}