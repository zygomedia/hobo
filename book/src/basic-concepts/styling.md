# Styling facilities

Most hobo components will be styled with either `.class()` or `.style()` functions, where either tuples or `css::style!()` macro is used. The naming is confusing somewhat, but the distinction is important.

* `.style()` and `.set_style()` use the `style` attribute, which can only take a bunch of properties without any selectors, so a plain tuple is used.
* `.class()`, `.set_class()` and `tagged` variants use the `class` attribute:
	* `css::style!()` uses a css-like `{ <selector> { <properties> } <selector> { <properties> } }` syntax
	* `css::class!()` is `css::style!(.& { <properties> })` or in other words - it's a bunch of properties applied just to the element being styled, similar to what can go in a `style` attribute, just using a class to refer to it.

For example, here's a style:

```rust
e::div()
    .class(css::style!(
        .& {
            css::height::px(393),
            css::display::flex,
            css::align_items::center,
            css::position::relative,
        }

        .& > svg {
            css::width::px(12),
            css::height::pct(100),
            css::cursor::pointer,
            css::flex_shrink::val(0),
            css::user_select::none,
        }

        .& > :not(:nth_child(1)) {
            css::z_index::val(200),
        }

        .& > div:not(:nth_child(1)) {
            css::width::pct(17.5),
            css::height::pct(100),
            css::display::flex,
            css::align_items::center,
        }

        .&.& > :nth_child(5) {
            css::width::pct(30),
        }

        .& > *:nth_child(3) > img,
        .& > *:nth_child(4) > img,
        .& > svg:last_child {
			css::transform::scale_x(-1),
        }

        .& >> img {
            css::height::pct(100),
        }
    ))
```

> **Chaining vs non-chaining syntax:** `.style()` is the chaining syntax, `.set_style()` is the non-chaining alternative. Similarly, `.class()` and `.set_class()`. More about chaining vs non-chaining syntax in [Building the DOM](../building-the-dom.md#chaining-vs-non-chaining-syntax).
