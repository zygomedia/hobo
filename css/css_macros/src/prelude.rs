pub use heck::*;
pub use itertools::Itertools;
pub use proc_quote::quote;
pub use syn::{
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
	Result, Token,
	// ext::IdentExt as _,
};
pub use proc_macro2::{Span, TokenStream, TokenTree};
pub use proc_macro_error::{proc_macro_error, abort};
pub use proc_macro_hack::proc_macro_hack;

pub fn css_crate_name() -> TokenStream {
	let into_ident = |x: String| syn::Ident::new(&x, Span::call_site());
	let hobo = proc_macro_crate::crate_name("hobo").ok().map(into_ident);
	let hobo_css = proc_macro_crate::crate_name("hobo-css").ok().map(into_ident);
	match (hobo, hobo_css) {
		(Some(hobo), _) => quote! { #hobo::css },
		(_, Some(hobo_css)) => quote! { #hobo_css },
		(None, None) => quote! { crate },
	}
}
