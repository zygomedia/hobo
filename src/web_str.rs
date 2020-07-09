#![allow(non_snake_case)]

// interning disabled in debug mode to help track memory leaks
macro_rules! intern_strings {
	() => {};
	($name:ident, $s:expr; $($rest:tt)*) => {
		pub fn $name() -> &'static str {
			#[cfg(debug_assertions)]
			{$s}

			#[cfg(not(debug_assertions))]
			{wasm_bindgen::intern($s)}
		}
		intern_strings! {$($rest)*}
	};
	($name:ident; $($rest:tt)*) => {
		pub fn $name() -> &'static str {
			#[cfg(debug_assertions)]
			{stringify!($name)}

			#[cfg(not(debug_assertions))]
			{wasm_bindgen::intern(stringify!($name))}
		}
		intern_strings! {$($rest)*}
	};
}

intern_strings! {
	class;
	r#type, "type";
	range;
	button;
	min; max; value; step;
	style;
	placeholder;
	src;
	href;
	disabled; selected;
	hidden;
	beforebegin; afterbegin; beforeend; afterend;
	checkbox; radio;
	accept;
	alt;
	checked;
	number;
	multiple;
	readonly;
	required;
	reversed;
	rows;
	tabindex;
	target;
	width; height;
	wrap;
	autofocus; autoplay;
	r#async, "async";
	autocomplete;
	download;
	draggable;
	dropzone;
	id;
	password;
	text;
	_blank;

	// events
	click; contextmenu; dblclick; mousedown; mouseenter;
	mouseleave; mousemove; mouseover; mouseout; mouseup;
	change; keydown; keyup; scroll; resize; blur; focus;

	// elements
	div; span; input; a; img; p;
	textarea; script; object; iframe; embed;
	select; option; nav;
	footer; address;
	h1; h2; h3; h4; h5; h6;
	ul; li; main; label;

	// svg elements
	svg; feColorMatrix; filter;
	circle; clipPath; defs;
	desc; ellipse; g;
	line; path; polygon;
	polyline; rect;
}
