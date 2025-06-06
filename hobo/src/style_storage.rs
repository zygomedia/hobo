use crate::{prelude::*, racy_cell::RacyCell};
use std::{collections::{HashMap, BTreeSet}, hash::{Hash, Hasher}};

#[derive(Default)]
pub struct StyleStorage {
	/// Used for checking whether a style already exists for reuse.
	///
	/// The hash is css::Style + Ordinal number,
	/// so the style is reused only per "class position".
	///
	/// See [fetch](Self::fetch) for both hashing and re-use checks.
	inserted_style_hashes: BTreeSet<u64>,

	/// Map representing the <style> elements in each window.
	///
	/// * key:     `String`                         - identifier, usually the name of the window.
	/// * value.0: `web_sys::Document`              - The main document of the window, to which head's <style> elements will be appended.
	/// * value.1: `Vec<web_sys::HtmlStyleElement>` - The <style> elements in this window's head.
	#[cfg(not(feature = "insert-rule"))]
	style_elements: HashMap<String, (web_sys::Document, Vec<web_sys::HtmlStyleElement>)>,

	#[cfg(feature = "insert-rule")]
	style_elements: HashMap<String, (web_sys::Document, Vec<web_sys::CssStyleSheet>)>,
}

#[expect(clippy::redundant_pub_crate)]
pub(crate) static STYLE_STORAGE: Lazy<RacyCell<StyleStorage>> = Lazy::new(|| RacyCell::new(StyleStorage {
	inserted_style_hashes: BTreeSet::new(),
	style_elements: hmap!["default".to_owned() => {
		let dom = web_sys::window().expect("no window").document().expect("no document");
		let head = dom.head().expect("dom has no head");
		let element = dom.create_element(web_str::style()).expect("can't create style element");
		head.append_child(&element).expect("can't append child");

		(
			dom,
			#[cfg(not(feature = "insert-rule"))] vec![element.unchecked_into::<web_sys::HtmlStyleElement>()],
			#[cfg(feature = "insert-rule")] vec![element.unchecked_into::<web_sys::HtmlStyleElement>().sheet().unwrap().unchecked_into::<web_sys::CssStyleSheet>()],
		)
	}],
}));

#[extend::ext]
impl css::Style {
	// replace the ClassPlaceholder with actual element class
	fn fixup_class_placeholders(&mut self, class: &str) {
		for rule in &mut self.0 {
			match rule {
				css::Rule::Style(style_rule) => {
					for selector_component in &mut (style_rule.0).0 {
						if *selector_component == css::selector::SelectorComponent::ClassPlaceholder {
							*selector_component = css::selector::SelectorComponent::Class(class.to_owned());
						}
					}
				},
				css::Rule::Media(_, style) => style.fixup_class_placeholders(class),
				css::Rule::FontFace(..) => {},
			}
		}
	}

	fn sort_properties(&mut self) {
		for rule in &mut self.0 {
			match rule {
				css::Rule::Style(style_rule) => {
					style_rule.1.sort();
				},
				css::Rule::Media(_, style) => style.sort_properties(),
				css::Rule::FontFace(..) => {},
			}
		}
	}
}

// it checks if the style is already inserted as css into <style>
// if yes, just returns the class name
// if no, inserts it into <style> and then returns the class name
impl StyleStorage {
	pub fn fetch(&mut self, mut style: css::Style, ordinal: usize) -> String {
		// if stable sort used on properties before hashing, then order of declarations would be preserved
		// but different elements that use the same properties in a different order would still reuse the same class
		// in other words, if you're specifying the same property multiple times to override it - that should still work
		// but the order of properties should no longer influence the hash result
		style.sort_properties();

		// u64 hash from style + ordinal
		let mut hasher = ahash::AHasher::default();
		style.hash(&mut hasher);
		ordinal.hash(&mut hasher);
		let id = hasher.finish();

		// recover class name
		let class = format!("s-{id:x}");

		// check if style exists in cache, in which case it's already inserted - just return class name
		if self.inserted_style_hashes.contains(&id) { return class; }

		// caching the style id
		self.inserted_style_hashes.insert(id);

		style.fixup_class_placeholders(&class);

		#[cfg(not(feature = "insert-rule"))]
		let style_string = style.to_string();

		// for each window
		for (dom, ordered_style_elements) in self.style_elements.values_mut() {
			if ordered_style_elements.get(ordinal).is_none() {
				let style_element = dom.create_element(web_str::style()).expect("can't create style element");
				let head = dom.head().expect("dom has no head");
				head.append_child(&style_element).expect("can't append child");

				#[cfg(not(feature = "insert-rule"))]
				ordered_style_elements.push(style_element.unchecked_into::<web_sys::HtmlStyleElement>());

				#[cfg(feature = "insert-rule")]
				ordered_style_elements.push(style_element.unchecked_into::<web_sys::HtmlStyleElement>().sheet().unwrap().unchecked_into::<web_sys::CssStyleSheet>());
			}

			// insert the stringified generated css into the style tag
			#[cfg(not(feature = "insert-rule"))]
			ordered_style_elements[ordinal].append_with_str_1(&style_string).expect("can't append css string");

			#[cfg(feature = "insert-rule")] {
				let sheet = &ordered_style_elements[ordinal];
				for rule in &style.0 {
					sheet.insert_rule(&rule.to_string()).unwrap();
				}
			}
		}

		class
	}

	#[cfg(not(feature = "insert-rule"))]
	pub fn unregister_window(&mut self, window_name: &str) {
		self.style_elements.remove(window_name);
	}

	#[cfg(not(feature = "insert-rule"))]
	pub fn register_window(&mut self, window: &web_sys::Window, window_name: &str) {
		let dom = window.document().expect("window has no dom");
		let head = dom.head().expect("dom has no head");

		// Re-create each existing <style> element from the default window into the new window
		for default_window_style_index in 0..self.style_elements.get("default").expect("no default window").1.len() {
			let new_style_element = dom.create_element(web_str::style()).expect("can't create style element");
			head.append_child(&new_style_element).expect("can't append child");

			// re-add all the already existing styles,
			// especially necessary for re-registering a previously closed window
			let style_element = &self.style_elements.get("default").expect("no default window").1[default_window_style_index];
			new_style_element.set_inner_html(&style_element.inner_html());
			self.style_elements.entry(window_name.to_owned()).or_insert((dom.clone(), Vec::new())).1.push(new_style_element.unchecked_into::<web_sys::HtmlStyleElement>());
		}
	}
}
