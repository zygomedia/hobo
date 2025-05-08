# Property

Most CSS properties will be expressed as tuples of anything that implements `hobo::css::AppendProperty`, which includes:

* `css::Property` such as created by the `css::<prop>()` family of functions (e.g. `css::width()`, `css::flex_shrink()`, etc)
* `Vec<css::Property>`
* `()`
* `&'static str` and `String` as escape hatches
* `FnOnce(&mut Vec<Property>)` for rare complex logic
* Other tuples of things that implement `hobo::css::AppendProperty`

Conditional property inclusion could be expressed as different `Vec<css::Property>` where one is empty, e.g.

```rust,noplaypen
(
    css::display::flex,
    if active {
        css::properties![css::background_color::rgba(0x00_00_FF_FF)]
    } else {
        css::properties![css::color::gray(0xA8)]
    },
	"--moz-user-input:none;".to_string(),
)
```

(we have to use `css::properties!` instead of `vec!` because all properties are of different types)

Or alternatively, by leveraging `FnOnce`

```rust,noplaypen
(
    css::display::flex,
    move |props| if active { props.push(css::background_color::rgba(0x00_00_FF_FF)); },
	"--moz-user-input:none;".to_string(),
)
```

