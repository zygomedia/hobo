# `hobo::create`

[This module](https://docs.rs/hobo/latest/hobo/create/index.html) has a `snake_case` function which returns a corresponding `PascalCase` concrete type that implements **AsElement** for each supported basic HTML and SVG element.

```rust,noplaypen
let some_div: hobo::create::Div = hobo::create::div();
```

**Element** has methods that aren't available on regular entities.

## `hobo::AsElement` and `hobo::AsEntity`

Sometimes it's useful to have custom types so you can have some special capabilities on your **Entities** or **Elements**.

```rust,noplaypen
#[derive(hobo::AsElement, Clone, Copy /* etc */)]
struct Checkbox(hobo::create::Div);

// just an example of why you might want to do this
impl Checkbox {
	fn is_checked(&self) -> bool {
		*self.get_cmp_or_default::<bool>()
	}

	fn set_checked(&self, checked: bool) {
		*self.get_cmp_mut_or_default::<bool>() = checked;
	}

	// probably etc methods
}
```

The `hobo::AsElement` derive macro expects either a tuple struct or a regular struct where the **Entity** field is named `element` e.g.

```rust,noplaypen
#[derive(hobo::AsElement, Clone, Copy /* etc */)]
struct CustomSelect {
	element: hobo::create::Select,
	// etc
}
```

## `Element` and type erasure

It's often useful to mix different types of **Elements**, for example:

```rust,noplaypen
fn content() -> impl hobo::AsElement {
	match tab {
		Tab::Main => main_page(), // hobo::create::Div
		Tab::Blogpost => article(), // hobo::create::Article
		// etc
	}
}
```

This won't compile, but the distinction between types in this case isn't useful. So we can erase the concrete types and get the general `Element`:

```rust,noplaypen
fn content() -> impl hobo::AsElement {
	match tab {
		Tab::Main => main_page().as_element(), // hobo::Element
		Tab::Blogpost => article().as_element(), // hobo::Element
		// etc
	}
}
```

If you have a regular **Entity** or something that at least implements `hobo::AsEntity` - you can recover **Element** capabilities by just wrapping it in an `Element`:

```rust,noplaypen
let elem = hobo::Element(some_entity);
```

This pattern is often useful when using queries to find elements, as queries often return entities (more on them in [queries](../state/queries.md))

```rust,noplaypen
let (_, entity) = hobo::find_ond::<With<ComponentFoo>, Entity>();
// We know that this entity is an Input element we've made,
// but we need it's type to be an Input, not Entity,
// to e.g. access it's value via the get/set_value methods
let input_element = hobo::create::Input(entity);
let input_value = input_element.value();
```

One can think of it almost as casting - we're fetching an entity which we, as the writer, know is an Input - however, we need to "cast" this Entity to an Input type in order to access Input capabilities.
