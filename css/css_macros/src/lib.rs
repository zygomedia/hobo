use heck::*;
use itertools::Itertools;
use proc_quote::quote;
use syn::{
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
	Result, Token,
	// ext::IdentExt as _,
};
use proc_macro2::{Span, TokenStream};
use proc_macro_error::{proc_macro_error, abort};
use proc_macro_hack::proc_macro_hack;

#[derive(Debug, Clone)]
struct HyphenatedName(String);

impl Parse for HyphenatedName {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self(<Punctuated<proc_macro2::TokenTree, Token![-]>>::parse_separated_nonempty(input)?.into_iter().map(|x| x.to_string()).join("-")))
	}
}

#[derive(Debug)]
struct Input {
	property: HyphenatedName,
	values: Vec<Value>,
}

impl Parse for Input {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut values = vec![
			Value::EnumVariant(HyphenatedName("initial".to_owned())),
			Value::EnumVariant(HyphenatedName("inherit".to_owned())),
			Value::EnumVariant(HyphenatedName("unset".to_owned())),
			Value::EnumVariant(HyphenatedName("revert".to_owned())),
		];
		let property = input.parse()?;

		while let Ok(value) = input.parse() {
			values.push(value);
		}

		Ok(Self { property, values })
	}
}

#[derive(Debug, Clone)]
enum Value {
	EnumVariant(HyphenatedName),
	Unit,
	String,
	Number,
	Float,
	Raw,
}

impl Parse for Value {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.peek(syn::token::Bracket) {
			syn::custom_keyword!(unit);
			syn::custom_keyword!(string);
			syn::custom_keyword!(number);
			syn::custom_keyword!(float);
			syn::custom_keyword!(raw);

			let content;
			syn::bracketed!(content in input);
			if content.parse::<unit>().is_ok() {
				return Ok(Self::Unit);
			} else if content.parse::<string>().is_ok() {
				return Ok(Self::String);
			} else if content.parse::<number>().is_ok() {
				return Ok(Self::Number);
			} else if content.parse::<float>().is_ok() {
				return Ok(Self::Float);
			} else if content.parse::<raw>().is_ok() {
				return Ok(Self::Raw);
			}
		} else if let Ok(x) = input.parse::<HyphenatedName>() {
			return Ok(Self::EnumVariant(x));
		}

		Err(input.error("unexpected tokens"))
	}
}

#[proc_macro]
pub fn easy_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as Input);

	let property_snek = proc_macro2::Ident::new(&input.property.0.to_snek_case(), Span::call_site());
	let property_camel = proc_macro2::Ident::new(&input.property.0.to_camel_case(), Span::call_site());

	let enum_members = input.values.iter().map(|value| match value {
		Value::EnumVariant(value) => {
			let value_camel = proc_macro2::Ident::new(&value.0.to_camel_case(), Span::call_site());
			quote! {#value_camel,}
		},
		Value::Unit => {
			quote! {
				Zero,
				Some(crate::units::Unit),
			}
		},
		Value::String => {
			quote! {String(String),}
		},
		Value::Raw => {
			quote! {Raw(String),}
		},
		Value::Number => {
			quote! {Number(i32),}
		},
		Value::Float => {
			quote! {Number(crate::units::F32),}
		},
	});

	let to_string_lines = input.values.iter().map(|value| match value {
		Value::EnumVariant(value) => {
			let value_camel = proc_macro2::Ident::new(&value.0.to_camel_case(), Span::call_site());
			let css_string = format!("{}:{};", input.property.0, value.0);
			quote! {Self::#value_camel => #css_string.to_owned(),}
		},
		Value::Unit => {
			let css_format_string = format!("{}:{{}};", input.property.0);
			let css_zero_string = format!("{}:0;", input.property.0);
			quote! {
				Self::Some(x) => format!(#css_format_string, x.to_string()),
				Self::Zero => #css_zero_string.to_owned(),
			}
		},
		Value::String => {
			let css_format_string = format!(r#"{}:"{{}}";"#, input.property.0);
			quote! {Self::String(x) => format!(#css_format_string, x),}
		},
		Value::Raw => {
			let css_format_string = format!("{}:{{}};", input.property.0);
			quote! {Self::Raw(x) => format!(#css_format_string, x),}
		},
		Value::Number | Value::Float => {
			let css_format_string = format!("{}:{{}};", input.property.0);
			quote! {Self::Number(x) => format!(#css_format_string, x),}
		},
	});

	let macro_values = input.values.iter().map(|value| match value {
		Value::EnumVariant(value) => {
			let value_camel = proc_macro2::Ident::new(&value.0.to_camel_case(), Span::call_site());
			let value_tt: TokenStream = syn::parse_str(&value.0).unwrap();
			quote! {(#value_tt) => { $crate::Property::#property_camel($crate::#property_camel::#value_camel) };}
		},
		Value::Unit => {
			quote! {
				(0) => { $crate::Property::#property_camel($crate::#property_camel::Zero) };
				($($val:tt)+) => { $crate::Property::#property_camel($crate::#property_camel::Some($crate::unit!($($val)+))) };
			}
		},
		Value::String => {
			quote! {($str:expr) => { $crate::Property::#property_camel($crate::#property_camel::String($str.into())) };}
		},
		Value::Raw => {
			quote! {($str:expr) => { $crate::Property::#property_camel($crate::#property_camel::Raw($str.into())) };}
		},
		Value::Number => {
			quote! {($num:expr) => { $crate::Property::#property_camel($crate::#property_camel::Number($num)) };}
		},
		Value::Float => {
			quote! {($num:expr) => { $crate::Property::#property_camel($crate::#property_camel::Number(unsafe { $crate::units::F32::unchecked_new($num as _) })) };}
		},
	});

	let res = quote!(
		#[derive(Debug, PartialEq, Eq, Hash, Clone)]
		pub enum #property_camel {
			#(#enum_members)*
		}

		impl ToString for #property_camel {
			fn to_string(&self) -> String {
				match self {
					#(#to_string_lines)*
				}
			}
		}

		#[macro_export]
		macro_rules! #property_snek {
			#(#macro_values)*
		}
	);

	// println!("{}", res.to_string());

	res.into()
}

#[proc_macro]
pub fn easy_color(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let property = input.into_iter().map(|x| x.to_string()).collect::<String>();

	let property_snek = proc_macro2::Ident::new(&property.to_snek_case(), Span::call_site());
	let property_camel = proc_macro2::Ident::new(&property.to_camel_case(), Span::call_site());

	let res = quote!(
		#[macro_export]
		macro_rules! #property_snek {
			(initial)       => {$crate::Property::#property_camel($crate::ColorValue::Initial)};
			(inherit)       => {$crate::Property::#property_camel($crate::ColorValue::Inherit)};
			(unset)         => {$crate::Property::#property_camel($crate::ColorValue::Unset)};
			(revert)        => {$crate::Property::#property_camel($crate::ColorValue::Revert)};
			(gray $c:expr)  => {$crate::Property::#property_camel($crate::ColorValue::Rgba($crate::Color { r: $c, g: $c, b: $c, a: 0xFF }))};
			(rgb $rgb:expr) => {$crate::Property::#property_camel($crate::ColorValue::Rgba(($rgb << 8 | 0xFF).into()))};
			($rgba:expr)    => {$crate::Property::#property_camel($crate::ColorValue::Rgba($rgba.into()))};
		}
	);

	res.into()
}

struct Selector(Vec<TokenStream>);

// TODO: I could factor out the literal quoted code if I return a Vec of some enum rather than just TokenStream
// and then each enum variant can have their own arguments etc
// in this way, parsing the selector and what code it outputs would be more cleanly seaprated
impl Parse for Selector {
	fn parse(input: ParseStream) -> Result<Self> {
		let crate_name = css_crate_name();
		let mut selector = Vec::new();

		while !input.is_empty() {
			let element = {
				if input.parse::<Token![+]>().is_ok() { quote! { .adjacent() } }
				else if input.parse::<Token![>>]>().is_ok() { quote! { .descendant() } }
				else if input.parse::<Token![>]>().is_ok() { quote! { .child() } }
				else if input.parse::<Token![,]>().is_ok() { quote! { .and() } }
				else if input.parse::<Token![*]>().is_ok() { quote! { .any() } }
				// html/svg element like div/span/a/p/img
				else if let Ok(element) = input.parse::<syn::Ident>() { quote! { .element(#crate_name::selector::Element::#element) } }
				else if input.parse::<Token![.]>().is_ok() {
					if input.peek(syn::token::Bracket) {
						// some element type
						let content = { let content; syn::bracketed!(content in input); content.parse::<syn::Type>()? };
						quote! { .class(<#content>::type_class_string()) }
					} else if input.peek(syn::token::Paren) {
						// class expr
						let content = { let content; syn::parenthesized!(content in input); content.parse::<syn::Expr>()? };
						quote! { .class(#content.into()) }
					} else if input.parse::<Token![&]>().is_ok() {
						quote! { .class_placeholder() }
					} else {
						abort!(input.parse::<proc_macro2::TokenTree>().unwrap(), "unknown token for a class")
					}
				} else if input.peek(syn::token::Bracket) {
					// literal attribute
					let content = { let content; syn::bracketed!(content in input); content.parse::<syn::Ident>()? };
					let content_str = content.to_string();
					quote! { .attribute(#content_str.into()) }
				} else if input.parse::<Token![#]>().is_ok() {
					// id expr
					let content = { let content; syn::parenthesized!(content in input); content.parse::<syn::Expr>()? };
					quote! { .id(#content.into()) }
				} else if input.parse::<Token![::]>().is_ok() {
					// pseudo element stuff
					let pseudo_element = input.parse::<syn::Ident>()?;
					quote! { .pseudo_element(#crate_name::selector::PseudoElement::#pseudo_element) }
				} else if input.parse::<Token![:]>().is_ok() {
					// pseudo class stuff
					syn::custom_keyword!(not);

					if input.peek(syn::token::Bracket) {
						let content = { let content; syn::bracketed!(content in input); content.parse::<syn::Expr>()? };
						quote! { .pseudo_class(#crate_name::selector::PseudoClass::raw(#content.into())) }
					} else if input.parse::<not>().is_ok() {
						let content = { let content; syn::parenthesized!(content in input); content.parse::<Selector>()? };
						quote! { .pseudo_class(#crate_name::selector::PseudoClass::not(#crate_name::selector::SelectorBuilder #content)) }
					} else if let Ok(pseudo_class) = input.parse::<syn::Ident>() {
						if input.peek(syn::token::Paren) {
							let content = { let content; syn::parenthesized!(content in input); content.parse::<TokenStream>()? };
							quote! { .pseudo_class(#crate_name::selector::PseudoClass::#pseudo_class(#content)) }
						} else {
							quote! { .pseudo_class(#crate_name::selector::PseudoClass::#pseudo_class) }
						}
					} else {
						abort!(input.parse::<proc_macro2::TokenTree>().unwrap(), "unknown token for a pseudo_class")
					}
				} else if input.parse::<Token![@]>().is_ok() {
					// at-rules
					if let Ok(at_name) = input.parse::<HyphenatedName>() {
						if at_name.0 == "font-face" { quote! { .font_face() } }
						else { abort!(input.parse::<proc_macro2::TokenTree>().unwrap(), "unknown at-rule") }
					} else {
						abort!(input.parse::<proc_macro2::TokenTree>().unwrap(), "unknown token for an at-rule")
					}
				} else {
					abort!(input.parse::<proc_macro2::TokenTree>().unwrap(), "unknown token")
				}
			};
			selector.push(element);
		}

		Ok(Self(selector))
	}
}

impl quote::ToTokens for Selector {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.0.iter().for_each(|x| x.to_tokens(tokens));
	}
}

fn css_crate_name() -> TokenStream {
	let into_ident = |x: String| syn::Ident::new(&x, Span::call_site());
	let hobo = proc_macro_crate::crate_name("hobo").ok().map(into_ident);
	let hobo_css = proc_macro_crate::crate_name("hobo-css").ok().map(into_ident);
	match (hobo, hobo_css) {
		(Some(hobo), _) => quote! { #hobo::css },
		(_, Some(hobo_css)) => quote! { #hobo_css },
		(None, None) => quote! { crate },
	}
}

#[proc_macro_error]
#[proc_macro_hack]
pub fn selector(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let crate_name = css_crate_name();
	let selector = syn::parse_macro_input!(input as Selector);

	// maybe move into selector's to tokens
	(quote! ({#crate_name::selector::Selector::from(#crate_name::selector::SelectorBuilder #selector)})).into()
}

struct UnitValueMacro {
	macro_name: syn::Ident,
	property_name: syn::Ident,
}

impl Parse for UnitValueMacro {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			macro_name: input.parse()?,
			property_name: input.parse()?,
		})
	}
}

#[proc_macro_error]
#[proc_macro]
pub fn unit_value_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let UnitValueMacro { macro_name, property_name } = syn::parse_macro_input!(input);
	(quote! {
		#[macro_export]
		macro_rules! #macro_name {
			(initial)     => {$crate::Property::#property_name($crate::UnitValue::Initial)};
			(inherit)     => {$crate::Property::#property_name($crate::UnitValue::Inherit)};
			(unset)       => {$crate::Property::#property_name($crate::UnitValue::Unset)};
			(revert)      => {$crate::Property::#property_name($crate::UnitValue::Revert)};
			(0)           => {$crate::Property::#property_name($crate::UnitValue::Zero)};
			($($val:tt)+) => {$crate::Property::#property_name($crate::UnitValue::Unit($crate::unit!($($val)+)))};
		}
	}).into()
}
