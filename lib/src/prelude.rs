#[doc(hidden)] pub use std::convert::TryInto;
#[doc(hidden)] pub use wasm_bindgen::JsCast;

pub use crate::{
	hierarchy::{Parent, Children},
	state,
	web_str,
	Entity,
	element::{
		Element,
		SomeElement,
	},
	AsEntity,
	storage::{
		Storage,
		DynStorage,
	},
	query::*,
	TypeClassString,
	dom_events::impls::*,
	create::StringValue,
};
pub(crate) use crate::{World, WORLD};
pub use crate::css::{self, AppendProperty, F32, F32Ext as _};
pub use wasm_bindgen::prelude::*;
pub use web_sys;

#[must_use]
pub fn default<T: Default>() -> T { T::default() }
