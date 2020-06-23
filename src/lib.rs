#![feature(proc_macro_hygiene, trait_alias, new_uninit)]

mod basic_element;
mod container;
pub mod create;
mod element;
mod enclose;
pub mod events;
pub mod prelude;
mod replaceable;
mod slot;
pub mod state_slice;
mod style_storage;
pub mod svg_create;
mod svg_element;
pub mod web_str;

pub use basic_element::{BasicElement, RawElement};
pub use container::*;
pub use create::{components as cmp, *};
pub use css;
pub use element::Element;
pub use events::*;
pub use hobo_derive::*;
pub use paste;
pub use replaceable::*;
pub use slot::*;
pub use svg_create::*;
pub use web_sys;

pub type Color = (u8, u8, u8, u8);

fn dom() -> web_sys::Document { web_sys::window().expect("no window").document().expect("no document") }

#[derive(Default)]
pub struct Context {
	style_storage: style_storage::StyleStorage,
}

thread_local! {
	static CONTEXT: Context = Default::default();
}
