# Getting Started

Here's a basic counter component:

```rust,noplaypen
use hobo::{
    prelude::*,
    create as e,
    signal::Mutable,
}

// <div class="s-f4d1763947b5e1ff">
//   <div>Counter value is: 0</div>
//   <button>increment</button>
//   <button>decrement</button>
// </div>

fn counter() -> impl hobo::AsElement {
    let counter_value = Mutable::new(0_i32);

    e::div()
        .class((
            // enum-like properties can also be set like `css::Display::Flex`
            css::display::flex,
            css::width::px(400),
            // #AA0000FF or #AA0000 or #A00 in normal css
            css::background_color::rgb(0xAA_00_00),
            css::align_items::center,
            css::justify_content::space_between,
        ))
        .child(e::div()
            .text_signal(counter_value.signal_ref(|value|
                format!("Counter value is: {value}"))
            )
        )
        .component(counter_value)
        .with(move |&counter_div| counter_div
            .child(e::button()
                .class(css::style!(
                    // .& is replaced with "current" class name, similar to SASS
                    // or styled-components
                    .& {
                        // shortcut for padding-left and padding-right
                        css::padding_horizontal::px(16),
                        css::background_color::rgba(css::colors::PALEVIOLETRED),
                    }

                    .&:hover {
                        css::background_color::rgba(css::colors::GREEN),
                    }
                ))
                .text("increment")
                .on_click(move |_| {
                    *counter_div.get_cmp::<Mutable<i32>>().lock_mut() += 1;
                })
            )
            .add_child(e::button() // same as .child but non-chaining
                // since this style is identical to the one above it -
                // the class will be reused to avoid copypasting -
                // the button generating code can be moved into
                // a function or maybe just the code that defines the style
                .class(css::style!(
                    .& {
                        css::padding_horizontal::px(16),
                        css::background_color::rgba(css::colors::PALEVIOLETRED),
                    }

                    .&:hover {
                        css::background_color::rgba(css::colors::GREEN),
                    }
                ))
                .text("decrement")
                .on_click(move |_| {
                    *counter_div.get_cmp::<Mutable<i32>>().lock_mut() -= 1;
                })
            )
        )
}
```
