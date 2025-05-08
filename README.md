# hobo

<a href="https://crates.io/crates/hobo"><img alt="Crate Info" src="https://img.shields.io/crates/v/hobo.svg"/></a>
<a href="https://docs.rs/hobo"><img alt="API Docs" src="https://img.shields.io/badge/docs.rs-hobo-yellow"/></a>

**hobo** is an opinionated, batteries-included Rust frontend framework. Works on **stable Rust**.    
**STILL WIP** although used in production by [Zygo Media](https://zygomedia.com).    
[Check out the Book](https://zygomedia.github.io/hobo)!

### Notable features:

* **no virtual DOM** - HTML elements are just components added to entities and can be accessed directly via `web_sys::HtmlElement`
* no Model-View-Update (aka Elm architecture) - state management is manual, usually via Entity-Component relations
* no HTML macros - just Rust functions
* built-in styling, kind of like CSS-in-JS except it's just Rust functions
* **minimal macros/DSLs** - just Rust functions, the only notable exception is CSS selectors, which are pretty close to base CSS
* **reactivity support** via [futures-signals](https://github.com/Pauan/rust-signals)
* Entity-Component based approach allowing flexible state propagation and cohesion between elements without coupling or a need for global store or state

### Sneak peek:
```rust,noplaypen
pub use hobo::{
    prelude::*,
    create as e,
    signals::signal::{Mutable, SignalExt}
};

fn counter() -> impl hobo::AsElement {
    let counter = Mutable::new(0);

    e::div()
        .class((
            css::display::flex,
            css::flex_direction::column,
            css::width::px(400),
        ))
        .child(e::div()
            .text_signal(counter.signal_ref(|value| {
                format!("Counter value is: {value}")
            }))
        )
        .child(e::button()
            .text("increment")
            .on_click(move |_| *counter.lock_mut() += 1)
        )
}
```

TODO:
### Comparions to other Rust-in-browser frameworks
* yew
* leptos
* seed
* etc
