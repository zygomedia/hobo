# Styling facilities

Most **Elements** will be styled with either `.class()` or `.style()` functions, where either `css::style!` or a property tuple will be used.

* `.style()` and `.set_style()` use the `style` attribute, which can only take a bunch of properties without any selectors, so a property tuple is used.
* `.class()`, `.set_class()` and `tagged` or `typed` variants use the `class` attribute:

For example, here's a style:

```rust,noplaypen
hobo::create::div()
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

        .& > :not(:nth_child(0, 1)) { // nth_child will convert to An+B syntax
            css::z_index::val(200),
        }

        .& > div:not(:nth_child(0, 1)) {
            css::width::pct(17.5),
            css::height::pct(100),
            css::display::flex,
            css::align_items::center,
        }

        // doubling up on the class name increases specificity
        .&.& > :nth_child(0, 5) { 
            css::width::pct(30),
        }

        .& > *:nth_child(0, 3) > img,
        .& > *:nth_child(0, 4) > img,
        .& > svg:last_child {
            css::transform::translate_x(50.),
        }

        .& >> img { // this is same as `.& img` selector in css
            css::height::pct(100),
        }
    ))
```

Property tuple example:

```rust,noplaypen
    hobo::create::div()
        .style((
            // Shortcut for same width and height
            css::size::px(12),
            css::display::flex,
        ))
```

If only a single property is used, one can omit the tuple:

```rust,noplaypen
    hobo::create::div()
        .class(css::display::flex)
```

> **Chaining vs non-chaining syntax:** `.style()` is the chaining syntax, `.set_style()` is the non-chaining alternative. Similarly, `.class()` and `.set_class()`. More about chaining vs non-chaining syntax in [Building the DOM](./building-the-dom.md#chaining-vs-non-chaining-syntax).
